
int __cdecl FUN_00592600(uint param_1,void *param_2,undefined4 *param_3)

{
  bool bVar1;
  uint uVar2;
  void *this;
  uint uVar3;
  int iVar4;
  uint uVar5;
  undefined4 *puVar6;
  uint uStack_1c;
  uint uStack_18;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_2;
  puStack_8 = &LAB_00650878;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar6 = param_3;
  uVar2 = param_1;
  *param_3 = 0;
  param_2 = (void *)FUN_00523450(param_1,this,param_3);
  if ((*(int *)(uVar2 + 4) == 0) || (*(int *)(uVar2 + 0xc) == 0)) goto LAB_00592830;
  if ((param_2 != (void *)0x0) && (*(int *)((int)this + 4) == -1)) {
    local_14 = 0x90;
    local_10 = 0x98;
    local_4 = 1;
    uVar3 = (**(code **)(**(int **)(uVar2 + 0x34) + 4))();
    if ((uVar3 < local_14) || (local_10 <= uVar3)) {
      uStack_1c = 0x98;
      uStack_18 = 0xa0;
      local_4._0_1_ = 2;
      uVar3 = (**(code **)(**(int **)(uVar2 + 0x34) + 4))();
      if ((uVar3 < uStack_1c) || (uStack_18 <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      puVar6 = param_3;
      if (bVar1) goto LAB_00592714;
      bVar1 = true;
    }
    else {
LAB_00592714:
      bVar1 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      local_14 = 0x40;
      local_10 = 0x10;
      local_4 = 3;
      FUN_00520580(this,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *puVar6 = 0xffffffff;
    }
  }
  if (((param_2 != (void *)0x0) && (*(int *)((int)this + 4) == -1)) &&
     (*(int *)(*(int *)(uVar2 + 0x14) + 0x70) != 0)) {
    uVar5 = 1;
    local_14 = 0x90;
    local_10 = 0x98;
    local_4 = 4;
    uVar3 = (**(code **)(**(int **)(uVar2 + 0x34) + 4))();
    if ((uVar3 < local_14) || (local_10 <= uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      uVar5 = *(uint *)(*(int *)(uVar2 + 0x34) + 0x88) >> 1 & 1;
    }
    puVar6 = param_3;
    if (uVar5 == 0) {
      local_14 = 0x40;
      local_10 = 0x11;
      local_4 = 5;
      FUN_00520580(this,&local_14);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_3 = 0xffffffff;
      puVar6 = param_3;
    }
  }
LAB_00592830:
  if (*(int *)(uVar2 + 0xc) != 0) {
    if (param_2 != (void *)0x0) {
      if (((*(int *)((int)this + 4) == -1) && (*(int *)(*(int *)(uVar2 + 0x14) + 0x78) != 0)) &&
         ((*(byte *)(*(int *)(uVar2 + 0x38) + 0x50) & 8) != 0)) {
        local_14 = 0x40;
        local_10 = 0x21;
        local_4 = 6;
        FUN_00520580(this,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar6 = 6;
      }
      if (param_2 != (void *)0x0) {
        if ((*(int *)((int)this + 4) == -1) &&
           ((*(byte *)(*(int *)(uVar2 + 0x38) + 0x50) & 0x10) != 0)) {
          local_14 = 0x40;
          local_10 = 0x22;
          local_4 = 7;
          FUN_00520580(this,&local_14);
          local_4 = 0xffffffff;
          FUN_00619730();
          *puVar6 = 6;
        }
        if ((param_2 != (void *)0x0) && (*(int *)((int)this + 4) == -1)) {
          FUN_004ece30(&param_1);
          local_4 = 8;
          FUN_004ece30(&uStack_1c);
          local_4 = CONCAT31(local_4._1_3_,9);
          param_2 = (void *)(**(code **)(**(int **)(uVar2 + 0x38) + 0xc))(&param_1);
          if (param_2 != (void *)0x0) {
            param_2 = (void *)(**(code **)(**(int **)(uVar2 + 0x34) + 0xc))(&uStack_1c);
          }
          iVar4 = FUN_004ece60(&param_1);
          if ((iVar4 == 0) || (param_1 != uStack_1c)) {
            local_14 = 0x40;
            local_10 = 0x20;
            local_4._0_1_ = 10;
            FUN_00520580(this,&local_14);
            local_4 = CONCAT31(local_4._1_3_,9);
            FUN_00619730();
            *puVar6 = 6;
          }
          local_4 = CONCAT31(local_4._1_3_,8);
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
    }
    if ((((*(int *)(uVar2 + 0xc) != 0) && (param_2 != (void *)0x0)) &&
        (*(int *)((int)this + 4) == -1)) && (*(int *)(*(int *)(uVar2 + 0x14) + 0x74) != 0)) {
      uVar5 = 0;
      local_14 = 0x90;
      local_10 = 0x98;
      local_4 = 0xb;
      uVar3 = (**(code **)(**(int **)(uVar2 + 0x34) + 4))();
      if ((uVar3 < local_14) || (local_10 <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        uVar5 = *(uint *)(*(int *)(uVar2 + 0x34) + 0x88) & 1;
      }
      if (uVar5 == 0) {
        local_14 = 0x40;
        local_10 = 0x16;
        local_4 = 0xc;
        FUN_00520580(this,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_3 = 0xd;
      }
    }
  }
  ExceptionList = pvStack_c;
  return (int)param_2;
}

