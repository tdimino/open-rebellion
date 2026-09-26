
int * __thiscall FUN_0045aac0(void *this,void *param_1,int param_2,uint param_3)

{
  int iVar1;
  int *this_00;
  uint *puVar2;
  void *pvVar3;
  int iVar4;
  BOOL BVar5;
  uint uVar6;
  uint local_2c;
  HWND local_28;
  undefined4 local_24;
  undefined1 auStack_20 [4];
  int iStack_1c;
  uint uStack_18;
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar3 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006308dc;
  pvStack_c = ExceptionList;
  local_24 = *(undefined4 *)((int)this + 0x1c);
  local_28 = *(HWND *)((int)this + 0x18);
  ExceptionList = &pvStack_c;
  FUN_0042d170(param_1,&local_2c);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  param_1 = (void *)0x0;
  switch(*(uint *)((int)pvVar3 + 0xc) >> 0x10) {
  default:
    uVar6 = 0;
    break;
  case 4:
    uVar6 = (local_2c & 0x3ff) << 6 | 9;
    break;
  case 8:
    uVar6 = (local_2c & 0x3ff) << 6 | 10;
    break;
  case 0x10:
    uVar6 = (local_2c & 0x3ff) << 6 | 4;
    break;
  case 0x40:
    uVar6 = (local_2c & 0x3ff) << 6 | 0xb;
  }
  this_00 = (int *)FUN_00604500((void *)(*(int *)((int)this + 0x194) + 0x6c),uVar6);
  if (this_00 == (int *)0x0) {
    MapWindowPoints(local_28,*(HWND *)(*(int *)((int)this + 0x194) + 0x18),(LPPOINT)&param_2,1);
    iVar4 = *(int *)((int)this + 0x194);
    iStack_1c = *(int *)(iVar4 + 0xcc);
    uStack_18 = *(uint *)(iVar4 + 0xd0);
    iStack_14 = *(int *)(iVar4 + 0xd4);
    iStack_10 = *(int *)(iVar4 + 0xd8);
    puVar2 = FUN_0042d170(pvVar3,auStack_20);
    local_4._0_1_ = 1;
    local_28 = (HWND)FUN_004f3220(*(int *)(*(int *)((int)this + 0x194) + 0x9c),puVar2);
    local_4._0_1_ = 0;
    FUN_00619730();
    switch(*(uint *)((int)pvVar3 + 0xc) >> 0x10) {
    case 4:
      pvVar3 = (void *)FUN_00618b70(0x26c);
      local_4._0_1_ = 2;
      if (pvVar3 == (void *)0x0) {
        param_1 = &DAT_00000002;
        local_4._0_1_ = 0;
        this_00 = (int *)0x0;
      }
      else {
        this_00 = FUN_00452fc0(pvVar3,local_24,param_2,param_3,0xe2,0x130,
                               *(int *)((int)this + 0x194),uVar6,local_28);
        local_4._0_1_ = 0;
        param_1 = &DAT_00000002;
      }
      break;
    case 8:
      pvVar3 = (void *)FUN_00618b70(0x184);
      local_4._0_1_ = 3;
      if (pvVar3 == (void *)0x0) {
        param_1 = (void *)0x8;
        local_4._0_1_ = 0;
        this_00 = (int *)0x0;
      }
      else {
        this_00 = FUN_004a7790(pvVar3,local_24,param_2,param_3,0xeb,0x130,
                               *(int *)((int)this + 0x194),uVar6,local_28);
        local_4._0_1_ = 0;
        param_1 = (void *)0x8;
      }
      break;
    case 0x10:
      pvVar3 = (void *)FUN_00618b70(0x1b4);
      local_4._0_1_ = 4;
      if (pvVar3 == (void *)0x0) {
        param_1 = (void *)0x4;
        local_4._0_1_ = 0;
        this_00 = (int *)0x0;
      }
      else {
        this_00 = FUN_004a2630(pvVar3,local_24,param_2,param_3,*(int *)((int)this + 0x194),uVar6,
                               local_28);
        local_4._0_1_ = 0;
        param_1 = (void *)0x4;
      }
      break;
    case 0x40:
      pvVar3 = (void *)FUN_00618b70(0x1cc);
      local_4._0_1_ = 5;
      if (pvVar3 == (void *)0x0) {
        this_00 = (int *)0x0;
      }
      else {
        this_00 = FUN_0049f130(pvVar3,local_24,param_2,param_3,*(uint *)((int)this + 0x194),uVar6,
                               local_28);
      }
      local_4._0_1_ = 0;
      param_1 = (void *)0x10;
    }
    iVar1 = iStack_14;
    iVar4 = iStack_1c;
    if (this_00 == (int *)0x0) goto LAB_0045aede;
    FUN_006071a0(this_00);
    this_00[0x14] = this_00[0x14] & 0xefffffff;
    FUN_00607ce0(this_00,2);
    FUN_005ffce0(this_00,0);
    if ((iVar4 <= param_2) && (iVar4 = param_2, iVar1 < param_2 + this_00[0xc])) {
      iVar4 = iVar1 - this_00[0xc];
    }
    param_2 = iVar4;
    uVar6 = uStack_18;
    if (((int)uStack_18 <= (int)param_3) &&
       (uVar6 = param_3, iStack_10 < (int)(param_3 + this_00[0xd]))) {
      uVar6 = iStack_10 - this_00[0xd];
    }
    param_3 = uVar6;
    SetWindowPos((HWND)this_00[6],(HWND)0x0,param_2,param_3,0,0,5);
    (**(code **)(*this_00 + 0x2c))(5);
    if (param_1 != (void *)0x0) {
      FUN_0041d2d0((uint)param_1);
    }
    FUN_0042ac70(*(void **)((int)this + 0x194),(int)this_00);
    FUN_005f4f10((void *)(*(int *)((int)this + 0x194) + 0x6c),(int)this_00);
    if (this_00 == (int *)0x0) goto LAB_0045aede;
  }
  iVar4 = FUN_004291d0(*(void **)((int)this + 0x194),this_00[9]);
  if (iVar4 == 0) {
    BVar5 = IsWindowVisible((HWND)this_00[6]);
    if (BVar5 == 0) {
      (**(code **)(*this_00 + 0x2c))(5);
    }
  }
  else {
    FUN_00429020(*(void **)((int)this + 0x194),iVar4);
  }
  SetWindowPos((HWND)this_00[6],(HWND)0x0,0,0,0,0,3);
LAB_0045aede:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return this_00;
}

