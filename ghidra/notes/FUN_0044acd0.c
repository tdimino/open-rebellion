
void __thiscall FUN_0044acd0(void *this,void *param_1,void *param_2)

{
  void *pvVar1;
  bool bVar2;
  void *pvVar3;
  uint uVar4;
  char *pcVar5;
  undefined *puVar6;
  undefined4 uVar7;
  undefined4 local_44;
  void *local_40;
  int local_3c;
  void *local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f410;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_24);
  bVar2 = false;
  local_4 = 0;
  FUN_005f2f50(local_18);
  local_40 = *(void **)((int)this + 300);
  local_4 = CONCAT31(local_4._1_3_,1);
  pvVar1 = *(void **)((int)local_40 + 0x1c);
  uVar4 = *(uint *)((int)this + 0x124) >> 0x18;
  local_3c = 0;
  local_44 = 0;
  local_30 = 0xa0;
  local_2c = 0xa2;
  if ((0x9f < uVar4) && (uVar4 < 0xa2)) {
    bVar2 = true;
  }
  local_38 = pvVar1;
  FUN_00619730();
  if (bVar2) {
    local_44 = CONCAT22(DAT_0065d424,0x1833);
    FUN_005f3010(local_24,&local_44);
    local_44 = FUN_0052c270(pvVar1,*(uint *)((int)pvVar1 + 0x24) >> 6 & 3,3);
  }
  else {
    local_30 = 0xa2;
    uVar4 = *(uint *)((int)this + 0x124) >> 0x18;
    local_2c = 0xa4;
    if ((uVar4 < 0xa2) || (0xa3 < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      local_44 = CONCAT22(DAT_0065d424,0x1829);
      FUN_005f3010(local_24,&local_44);
      local_44 = FUN_0052c8c0(pvVar1,*(uint *)((int)pvVar1 + 0x24) >> 6 & 3,3);
    }
    else {
      local_30 = 0xa4;
      uVar4 = *(uint *)((int)this + 0x124) >> 0x18;
      local_2c = 0xa6;
      if ((uVar4 < 0xa4) || (0xa5 < uVar4)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        local_44 = CONCAT22(DAT_0065d424,0x1831);
        FUN_005f3010(local_24,&local_44);
        local_44 = FUN_0052c5a0(pvVar1,*(uint *)((int)pvVar1 + 0x24) >> 6 & 3,3);
        local_3c = 1;
      }
    }
  }
  pvVar1 = (void *)((int)this + 0x130);
  FUN_00403e90(pvVar1,0x11);
  *(undefined4 *)((int)this + 0x134) = 0x24;
  FUN_00601b30(pvVar1,0xf,0x12);
  pcVar5 = (char *)FUN_00583c40((int)local_24);
  FUN_00601aa0(pvVar1,pcVar5);
  pvVar3 = param_2;
  pvVar1 = param_1;
  FUN_0044a210((int)this);
  param_2 = (void *)CONCAT22(DAT_0065d424,0x8664);
  FUN_005f3010(local_24,&param_2);
  FUN_0044b1d0(2,(int)local_24,pvVar1);
  FUN_005f3c20(local_24);
  puVar6 = FUN_004f62d0(*(int *)(*(int *)((int)this + 300) + 0x1c));
  pcVar5 = (char *)FUN_00583c40((int)puVar6);
  FUN_005f35b0(&local_30,pcVar5);
  local_4._0_1_ = 2;
  FUN_005f30d0(local_24,(int)&local_30);
  local_4._0_1_ = 1;
  FUN_005f2ff0(&local_30);
  FUN_0044b1d0(2,(int)local_24,pvVar3);
  FUN_005f3c20(local_24);
  param_2 = (void *)CONCAT22(DAT_0065d424,0x8666);
  FUN_005f3010(local_24,&param_2);
  uVar7 = 3;
  FUN_0044b1d0(3,(int)local_24,pvVar1);
  FUN_005f3c20(local_24);
  if (local_44 == 0) {
    param_2 = (void *)CONCAT22(DAT_0065d424,0x8772);
    FUN_005f3010(local_24,&param_2);
  }
  else if (*(int *)((int)local_40 + 0x58) == 0) {
    param_2 = (void *)CONCAT22(DAT_0065d424,0x8769);
    FUN_005f3010(local_24,&param_2);
    FUN_0044b1d0(3,(int)local_24,pvVar3);
  }
  else {
    if (local_3c == 0) {
      param_2 = (void *)CONCAT22(DAT_0065d424,0x8770);
    }
    else {
      param_2 = (void *)CONCAT22(DAT_0065d424,0x8771);
    }
    FUN_005f3010(local_24,&param_2);
    FUN_0044b1d0(3,(int)local_24,pvVar3);
    FUN_005f3c20(local_24);
    param_2 = (void *)CONCAT22(DAT_0065d424,0x8768);
    FUN_005f3010(local_24,&param_2);
    uVar7 = 4;
    FUN_0044b1d0(4,(int)local_24,pvVar1);
    FUN_005f3c20(local_24);
    FUN_005f31a0(local_24,*(undefined4 *)((int)local_40 + 0x58));
    FUN_0044b1d0(4,(int)local_24,pvVar3);
    if ((*(int *)((int)local_40 + 0x58) != 0) &&
       (((*(uint *)((int)local_38 + 0x24) ^ *(uint *)((int)local_40 + 0x24)) & 0xc0) == 0)) {
      FUN_005f3c20(local_24);
      param_2 = (void *)CONCAT22(DAT_0065d424,0x8765);
      FUN_005f3010(local_24,&param_2);
      uVar7 = 5;
      FUN_0044b1d0(5,(int)local_24,pvVar1);
      local_38 = (void *)0x1;
      local_34 = 0xffffffff;
      local_4._0_1_ = 3;
      param_1 = (void *)0x0;
      FUN_004fd280(local_40,&local_38,(int *)&param_1);
      FUN_005f3c20(local_24);
      FUN_005f31a0(local_24,param_1);
      FUN_0044b1d0(5,(int)local_24,pvVar3);
      local_4._0_1_ = 1;
      FUN_00619730();
    }
  }
  FUN_0044b1d0(uVar7,(int)local_24,pvVar3);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  ExceptionList = local_c;
  return;
}

