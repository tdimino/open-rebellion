
uint __thiscall FUN_004fca30(void *this,void *param_1)

{
  uint uVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar5;
  undefined3 extraout_var_01;
  void *this_00;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  uint *puVar6;
  uint *puVar7;
  uint uVar8;
  void *pvVar9;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640238;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar3 = FUN_0053a010((int)this);
  bVar4 = FUN_005406d0(this);
  if (CONCAT31(extraout_var_00,bVar4) == 0) {
    ExceptionList = local_c;
    return CONCAT31(extraout_var,bVar3);
  }
  if (CONCAT31(extraout_var,bVar3) == 0) {
    iVar5 = 0;
    pvVar9 = param_1;
  }
  else {
    iVar5 = FUN_00539fd0(this,0);
    pvVar9 = *(void **)(iVar5 + 0x1c);
    bVar3 = FUN_00539ff0((int)pvVar9);
    iVar5 = CONCAT31(extraout_var_01,bVar3);
  }
  this_00 = param_1;
  if (iVar5 != 0) {
    this_00 = (void *)FUN_00539fd0(pvVar9,*(uint *)((int)this + 0x24) >> 4 & 3);
    bVar3 = FUN_0053a010((int)this_00);
    if ((CONCAT31(extraout_var_02,bVar3) != 0) &&
       (bVar3 = FUN_0053a010((int)this), CONCAT31(extraout_var_03,bVar3) != 0)) {
      pvVar9 = *(void **)((int)this + 0x1c);
      bVar3 = FUN_00539ff0((int)pvVar9);
      uVar8 = CONCAT31(extraout_var_04,bVar3);
      goto LAB_004fcad0;
    }
  }
  uVar8 = 0;
  pvVar9 = param_1;
LAB_004fcad0:
  if (uVar8 != 0) {
    puVar6 = FUN_004025b0(pvVar9,&local_10);
    local_4 = 0;
    puVar7 = FUN_004025b0(this_00,&local_14);
    uVar1 = *puVar7;
    uVar2 = *puVar6;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar1 != uVar2) {
      uVar8 = FUN_004fcb50(this,param_1);
    }
  }
  ExceptionList = local_c;
  return uVar8;
}

