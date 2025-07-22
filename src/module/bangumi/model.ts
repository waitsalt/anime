// 用户公开信息
interface UserPublic {
  id: number;
  url: string;
  username: string;
  nickname: string;
  user_group: UserGroup;
  avatar: Avatar;
  sign: string;
}

// 用户完整信息
interface User extends UserPublic {
  email: string;
  reg_time: string;
  time_offset: number;
}

// 用户头像
interface Avatar {
  large: string;
  medium: string;
  small: string;
}

// 头像类型
enum AvatarType {
  Small = "small",
  Medium = "medium",
  Large = "large",
}

// 用户组
enum UserGroup {
  Admin = 1,
  BangumiAdmin = 2,
  DoujinAdmin = 3,
  MutedUser = 4,
  BlockedUser = 5,
  WikiAdmin = 9,
  User = 10,
  WikiUser = 11,
}

// 统一导出所有类型和工具函数
export type { UserPublic, User, Avatar, AvatarType, UserGroup };
