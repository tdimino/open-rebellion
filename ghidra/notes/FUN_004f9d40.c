
uint __thiscall FUN_004f9d40(void *this,void *param_1)

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
  undefined3 extraout_var_05;
  uint uVar8;
  void *pvVar9;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ff78;
  local_c = ExceptionList;
  bVar3 = false;
  ExceptionList = &local_c;
  bVar4 = FUN_0053a010((int)this);
  uVar8 = CONCAT31(extraout_var,bVar4);
  bVar4 = FUN_005406d0(this);
  if (CONCAT31(extraout_var_00,bVar4) == 0) goto LAB_004f9ed1;
  if (uVar8 == 0) {
    iVar5 = 0;
    pvVar9 = param_1;
  }
  else {
    iVar5 = FUN_00539fd0(this,0);
    pvVar9 = *(void **)(iVar5 + 0x1c);
    bVar4 = FUN_00539ff0((int)pvVar9);
    iVar5 = CONCAT31(extraout_var_01,bVar4);
  }
  this_00 = param_1;
  if (iVar5 == 0) {
LAB_004f9de4:
    uVar8 = 0;
    pvVar9 = param_1;
  }
  else {
    this_00 = (void *)FUN_00539fd0(pvVar9,*(uint *)((int)this + 0x24) >> 4 & 3);
    bVar4 = FUN_0053a010((int)this_00);
    if ((CONCAT31(extraout_var_02,bVar4) == 0) ||
       (bVar4 = FUN_0053a010((int)this), CONCAT31(extraout_var_03,bVar4) == 0)) goto LAB_004f9de4;
    pvVar9 = *(void **)((int)this + 0x1c);
    bVar4 = FUN_00539ff0((int)pvVar9);
    uVar8 = CONCAT31(extraout_var_04,bVar4);
  }
  if (uVar8 == 0) {
    ExceptionList = local_c;
    return 0;
  }
  puVar6 = FUN_004025b0(pvVar9,&local_10);
  local_4 = 0;
  puVar7 = FUN_004025b0(this_00,&local_14);
  uVar2 = *puVar7;
  uVar1 = *puVar6;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (uVar2 != uVar1) {
    iVar5 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar5 == 0) {
      iVar5 = FUN_005404d0(pvVar9,*(uint *)((int)param_1 + 8));
      if (iVar5 == 0) goto LAB_004f9ed1;
      uVar8 = FUN_004fca30(this,param_1);
    }
    else {
      bVar3 = true;
      bVar4 = FUN_004fc600(this_00,this,param_1);
      uVar8 = CONCAT31(extraout_var_05,bVar4);
      if (uVar8 == 0) {
        ExceptionList = local_c;
        return 0;
      }
      iVar5 = FUN_004fd3d0();
      if ((iVar5 == 0) ||
         (iVar5 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3), iVar5 == 0))
      goto LAB_004f9ed1;
      uVar8 = (**(code **)(*(int *)this + 0x15c))(pvVar9,this_00,param_1);
    }
    bVar3 = true;
  }
LAB_004f9ed1:
  if ((uVar8 != 0) && (bVar3)) {
    uVar8 = FUN_004fc710(this,param_1);
  }
  ExceptionList = local_c;
  return uVar8;
}

