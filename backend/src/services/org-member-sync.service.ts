import { prisma } from '../lib/db.js';

export interface OrgMemberTag {
  id: string;
  name: string;
  color?: string;
}

export interface OrgMemberWithTags {
  memberAddress: string;
  role: string;
  tags: OrgMemberTag[];
  addedBy: string;
  createdAt: Date;
}

export class OrgMemberSyncService {
  /**
   * Sync organization members with tagging support.
   * Allows bulk operations for team organization.
   */
  async syncMembers(
    orgAddress: string,
    members: Array<{
      memberAddress: string;
      role: string;
      tags?: string[];
    }>,
  ): Promise<void> {
    for (const member of members) {
      await prisma.organizationMember.upsert({
        where: {
          orgAddress_memberAddress: {
            orgAddress,
            memberAddress: member.memberAddress,
          },
        },
        create: {
          orgAddress,
          memberAddress: member.memberAddress,
          role: member.role,
          isActive: true,
          tags: member.tags || [],
        },
        update: {
          role: member.role,
          tags: member.tags || [],
          isActive: true,
        },
      });
    }
  }

  /**
   * Add tags to a member.
   */
  async addTagsToMember(
    orgAddress: string,
    memberAddress: string,
    tags: string[],
  ): Promise<void> {
    const member = await prisma.organizationMember.findUnique({
      where: {
        orgAddress_memberAddress: {
          orgAddress,
          memberAddress,
        },
      },
    });

    if (!member) {
      throw new Error('Member not found');
    }

    const existingTags = (member.tags as string[]) || [];
    const newTags = Array.from(new Set([...existingTags, ...tags]));

    await prisma.organizationMember.update({
      where: {
        orgAddress_memberAddress: {
          orgAddress,
          memberAddress,
        },
      },
      data: {
        tags: newTags,
      },
    });
  }

  /**
   * Remove tags from a member.
   */
  async removeTagsFromMember(
    orgAddress: string,
    memberAddress: string,
    tags: string[],
  ): Promise<void> {
    const member = await prisma.organizationMember.findUnique({
      where: {
        orgAddress_memberAddress: {
          orgAddress,
          memberAddress,
        },
      },
    });

    if (!member) {
      throw new Error('Member not found');
    }

    const existingTags = (member.tags as string[]) || [];
    const updatedTags = existingTags.filter((tag) => !tags.includes(tag));

    await prisma.organizationMember.update({
      where: {
        orgAddress_memberAddress: {
          orgAddress,
          memberAddress,
        },
      },
      data: {
        tags: updatedTags,
      },
    });
  }

  /**
   * Get members by tag.
   */
  async getMembersByTag(orgAddress: string, tag: string): Promise<OrgMemberWithTags[]> {
    const members = await prisma.organizationMember.findMany({
      where: {
        orgAddress,
        isActive: true,
      },
    });

    return members
      .filter((m) => {
        const tags = (m.tags as string[]) || [];
        return tags.includes(tag);
      })
      .map((m) => ({
        memberAddress: m.memberAddress,
        role: m.role,
        tags: ((m.tags as string[]) || []).map((t) => ({ id: t, name: t })),
        addedBy: m.addedBy,
        createdAt: m.createdAt,
      }));
  }

  /**
   * List all members with their tags.
   */
  async listMembersWithTags(orgAddress: string): Promise<OrgMemberWithTags[]> {
    const members = await prisma.organizationMember.findMany({
      where: {
        orgAddress,
        isActive: true,
      },
      orderBy: { createdAt: 'asc' },
    });

    return members.map((m) => ({
      memberAddress: m.memberAddress,
      role: m.role,
      tags: ((m.tags as string[]) || []).map((t) => ({ id: t, name: t })),
      addedBy: m.addedBy,
      createdAt: m.createdAt,
    }));
  }

  /**
   * Get all unique tags in an organization.
   */
  async getOrgTags(orgAddress: string): Promise<string[]> {
    const members = await prisma.organizationMember.findMany({
      where: {
        orgAddress,
        isActive: true,
      },
      select: { tags: true },
    });

    const allTags = new Set<string>();
    members.forEach((m) => {
      const tags = (m.tags as string[]) || [];
      tags.forEach((t) => allTags.add(t));
    });

    return Array.from(allTags).sort();
  }
}

export const orgMemberSyncService = new OrgMemberSyncService();
