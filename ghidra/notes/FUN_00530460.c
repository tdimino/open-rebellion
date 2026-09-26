
int __thiscall FUN_00530460(void *this,void *param_1)

{
  void *this_00;
  void *this_01;
  void *this_02;
  void *this_03;
  int iVar1;
  bool bVar2;
  int local_14;
  int local_10;
  int local_c;
  int local_8;
  
  local_14 = 1;
  local_10 = 0;
  local_c = 0;
  local_8 = 0;
  for (this_00 = (void *)FUN_00505ef0(); this_00 != (void *)0x0;
      this_00 = (void *)FUN_0051a5d0(this_00)) {
    bVar2 = true;
    if (local_14 == 0) goto LAB_00530547;
    if (((*(uint *)((int)this + 0x24) ^ *(uint *)((int)this_00 + 0x24)) & 0xc0) == 0) {
      this_01 = (void *)FUN_00509670(this_00,0);
      this_02 = (void *)FUN_00509670(this_00,2);
      this_03 = (void *)FUN_00509670(this_00,1);
      if (((this_01 == (void *)0x0) || (this_02 == (void *)0x0)) ||
         (local_14 = 1, this_03 == (void *)0x0)) {
        local_14 = 0;
      }
      if (local_14 != 0) {
        iVar1 = FUN_00528040(this_01,0);
        local_10 = local_10 + iVar1;
        iVar1 = FUN_00528040(this_02,0);
        local_c = local_c + iVar1;
        iVar1 = FUN_00528040(this_03,0);
        local_8 = local_8 + iVar1;
      }
    }
  }
  bVar2 = local_14 == 0;
LAB_00530547:
  if (!bVar2) {
    local_14 = FUN_0052f130(this,*(int *)((int)this + 0x90) + local_10,param_1);
  }
  if ((local_14 != 0) &&
     (local_14 = FUN_0052f1a0(this,*(int *)((int)this + 0x94) + local_c,param_1), local_14 != 0)) {
    local_14 = FUN_0052f210(this,*(int *)((int)this + 0x98) + local_8,param_1);
  }
  return local_14;
}

