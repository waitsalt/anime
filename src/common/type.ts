enum CollectionType {
  WISH, // 想看
  DOING, // 再看
  DONE, // 看过
  ON_HOLD, // 搁置
  DROPPED, // 抛弃
  NOT_COLLECTED, // 未收藏
}

interface User {
  // 不同尺寸的头像
  avatar: {
    large: string; // 大
    medium: string; // 中
    small: string; // 小
  };
  sign: string; // 用户签名
  username: string; // 用户名
  nickname: string; // 用户昵称
  id: number; // 用户id
  //  1 = 管理员 - 2 = Bangumi 管理猿 - 3 = 天窗管理猿 - 4 = 禁言用户 - 5 = 禁止访问用户 - 8 = 人物管理猿 - 9 = 维基条目管理猿 - 10 = 用户 - 11 = 维基人
  user_group: number; // 用户所在权限组
}

export type { CollectionType, User };
