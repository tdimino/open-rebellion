
void __thiscall FUN_00445c60(void *this,void *param_1,void *param_2)

{
  uint uVar1;
  bool bVar2;
  char *pcVar3;
  int *piVar4;
  uint *puVar5;
  uint *puVar6;
  uint uVar7;
  int iVar8;
  undefined *puVar9;
  int *this_00;
  void *pvVar10;
  int iVar11;
  uint local_6c;
  undefined4 local_68;
  undefined4 local_64;
  int local_60;
  int local_5c;
  undefined4 local_58;
  uint local_54 [6];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ef88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_3c);
  this_00 = (int *)0x0;
  local_4 = 0;
  local_58 = CONCAT22(DAT_0065d424,0x8676);
  FUN_005f3010(local_3c,&local_58);
  pcVar3 = (char *)FUN_00583c40((int)local_3c);
  FUN_00601aa0((void *)((int)this + 0x130),pcVar3);
  FUN_005f2f50(local_18);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_0044a210((int)this);
  local_5c = 0;
  local_58 = 0;
  piVar4 = FUN_004f30a0(*(int *)(*(int *)((int)this + 0x114) + 0x9c));
  while (piVar4 != (int *)0x0) {
    puVar5 = FUN_004025b0(*(void **)((int)this + 300),local_54);
    local_4._0_1_ = 3;
    puVar6 = FUN_0042d170(piVar4,&local_68);
    uVar7 = *puVar6;
    uVar1 = *puVar5;
    FUN_00619730();
    local_4._0_1_ = 1;
    FUN_00619730();
    if (uVar7 == uVar1) {
      if (this_00 == (int *)0x0) {
        this_00 = piVar4;
      }
      local_5c = local_5c + 1;
      if ((*(byte *)(piVar4 + 0x1e) & 1) != 0) {
        local_58 = local_58 + 1;
      }
    }
    local_54[3] = 0x30;
    local_54[4] = 0x40;
    local_4._0_1_ = 2;
    piVar4 = (int *)FUN_004f6010(piVar4,local_54 + 3,1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_68 = CONCAT22(DAT_0065d424,0x8677);
  FUN_005f3010(local_3c,&local_68);
  iVar11 = 2;
  FUN_0044b1d0(2,(int)local_3c,param_1);
  FUN_004ece30(&local_6c);
  local_4._0_1_ = 4;
  iVar8 = *(int *)(*(int *)((int)this + 0x114) + 0x9c);
  if (iVar8 == 1) {
    uVar7 = 1;
  }
  else if (iVar8 == 2) {
    uVar7 = 2;
  }
  else {
    uVar7 = 0;
  }
  if (this_00 != (int *)0x0) {
    if (((uint)this_00[9] >> 6 & 3) == uVar7) {
      FUN_004f26d0(&local_6c,(undefined4 *)(*(int *)((int)this + 300) + 0x70));
    }
    else {
      puVar5 = FUN_0042d170(this_00,local_54);
      local_68 = 0x50;
      local_64 = 0x60;
      local_4._0_1_ = 5;
      if ((*puVar5 >> 0x18 < 0x50) || (0x5f < *puVar5 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 4;
      FUN_00619730();
      if (bVar2) {
        FUN_004ece30(&local_68);
        local_4 = CONCAT31(local_4._1_3_,6);
        (**(code **)(*this_00 + 0xc))(&local_68);
        local_54[3] = 0x90;
        local_54[4] = 0x98;
        if ((local_68 >> 0x18 < 0x90) || (0x97 < local_68 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          FUN_004f26d0(&local_6c,&local_68);
        }
        local_4._0_1_ = 4;
        FUN_00619730();
      }
    }
  }
  piVar4 = (int *)0x0;
  iVar8 = FUN_004ece60(&local_6c);
  if (iVar8 != 0) {
    piVar4 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x114) + 0x9c),&local_6c);
  }
  FUN_005f3c20(local_3c);
  if (piVar4 == (int *)0x0) {
    local_68 = CONCAT22(DAT_0065d424,0x8527);
    FUN_005f3010(local_3c,&local_68);
  }
  else {
    puVar9 = FUN_004f62d0((int)piVar4);
    pcVar3 = (char *)FUN_00583c40((int)puVar9);
    FUN_005f35b0(local_54 + 3,pcVar3);
    local_4._0_1_ = 7;
    FUN_005f30d0(local_3c,(int)(local_54 + 3));
    local_4._0_1_ = 4;
    FUN_005f2ff0(local_54 + 3);
  }
  pvVar10 = param_2;
  FUN_0044b1d0(2,(int)local_3c,param_2);
  local_54[1] = 1;
  local_54[2] = -1;
  local_4._0_1_ = 8;
  local_60 = 0;
  if (this_00 != (int *)0x0) {
    FUN_004fd2b0(this_00,(int *)(local_54 + 1),&local_60);
  }
  if ((local_54[2] == -1) && (local_60 != 0)) {
    local_68._0_2_ = 0x3814;
    local_68._2_2_ = DAT_0065d424;
    FUN_005f2fc0(local_24,&local_68);
    local_4._0_1_ = 9;
    FUN_005f3c20(local_3c);
    local_68 = CONCAT22(local_68._2_2_,0x8766);
    FUN_005f2fc0(local_30,&local_68);
    local_4._0_1_ = 10;
    pcVar3 = (char *)FUN_00583c40((int)local_30);
    FUN_005f35b0(local_54 + 3,pcVar3);
    local_4._0_1_ = 0xb;
    FUN_005f30d0(local_3c,(int)(local_54 + 3));
    local_4._0_1_ = 10;
    FUN_005f2ff0(local_54 + 3);
    iVar11 = 3;
    FUN_0044b1d0(3,(int)local_3c,param_1);
    FUN_005f3c20(local_3c);
    pcVar3 = (char *)FUN_00583c40((int)local_24);
    FUN_005f35b0(local_54 + 3,pcVar3);
    local_4._0_1_ = 0xc;
    FUN_005f30d0(local_3c,(int)(local_54 + 3));
    FUN_005f31a0(local_3c,local_60);
    local_4._0_1_ = 10;
    FUN_005f2ff0(local_54 + 3);
    FUN_0044b1d0(3,(int)local_3c,param_2);
    local_4._0_1_ = 9;
    FUN_005f2ff0(local_30);
    local_4._0_1_ = 8;
    FUN_005f2ff0(local_24);
    pvVar10 = param_2;
  }
  FUN_004f3220(*(int *)(*(int *)((int)this + 0x114) + 0x9c),
               (uint *)(*(int *)((int)this + 300) + 0x78));
  param_2 = (void *)CONCAT22(DAT_0065d424,0x8678);
  FUN_005f3010(local_3c,&param_2);
  FUN_0044b1d0(iVar11 + 1,(int)local_3c,param_1);
  FUN_005f3c20(local_3c);
  FUN_005f31f0(local_3c,local_5c);
  FUN_0044b1d0(iVar11 + 1,(int)local_3c,pvVar10);
  param_2 = (void *)CONCAT22(DAT_0065d424,0x8679);
  FUN_005f3010(local_3c,&param_2);
  FUN_0044b1d0(iVar11 + 2,(int)local_3c,param_1);
  FUN_005f3c20(local_3c);
  FUN_005f31f0(local_3c,local_58);
  FUN_0044b1d0(iVar11 + 2,(int)local_3c,pvVar10);
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_3c);
  ExceptionList = local_c;
  return;
}

