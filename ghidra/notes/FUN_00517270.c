
undefined4 __fastcall FUN_00517270(int param_1)

{
  void *pvVar1;
  char cVar2;
  char cVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  bool bVar7;
  bool bVar8;
  undefined3 extraout_var;
  int iVar9;
  void *this;
  void *pvVar10;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  int iVar11;
  undefined3 extraout_var_04;
  uint local_44;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006428d0;
  local_c = ExceptionList;
  pvVar1 = (void *)(param_1 + 0x90);
  local_44 = 0;
  cVar3 = '\0';
  ExceptionList = &local_c;
  bVar4 = FUN_005197f0(param_1);
  if (CONCAT31(extraout_var,bVar4) == 0) {
    ExceptionList = local_c;
    return 0;
  }
  iVar9 = FUN_004ece60((uint *)(param_1 + 0x8c));
  this = (void *)FUN_00505f60((uint *)(param_1 + 0x8c));
  if ((this == (void *)0x0) || (bVar4 = true, iVar9 == 0)) {
    bVar4 = false;
  }
  if (this == (void *)0x0) goto LAB_005174c3;
  pvVar10 = FUN_004ffef0(local_2c,this,1,1);
  local_4 = 0;
  bVar5 = FUN_005131b0((int)pvVar10);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  pvVar10 = FUN_00503a50(local_2c,this,1,1);
  local_4 = 1;
  bVar6 = FUN_005131b0((int)pvVar10);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  pvVar10 = FUN_004ffef0(local_2c,this,1,2);
  local_4 = 2;
  bVar7 = FUN_005131b0((int)pvVar10);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  pvVar10 = FUN_00503a50(local_2c,this,1,2);
  local_4 = 3;
  bVar8 = FUN_005131b0((int)pvVar10);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  if ((CONCAT31(extraout_var_00,bVar5) == 0) && (CONCAT31(extraout_var_01,bVar6) == 0)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  if ((CONCAT31(extraout_var_02,bVar7) == 0) && (CONCAT31(extraout_var_03,bVar8) == 0)) {
    cVar2 = false;
  }
  else {
    cVar2 = true;
  }
  if (((bVar5) && ((bool)cVar2)) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  if (bVar5) {
    if (!(bool)cVar2) {
      if (!bVar5) goto LAB_00517400;
      local_44 = 1;
      cVar2 = '\x02';
      goto LAB_00517418;
    }
    local_44 = 0;
  }
  else {
LAB_00517400:
    if ((bool)cVar2) {
      local_44 = 2;
    }
LAB_00517418:
    iVar9 = FUN_0050a040(this,local_44,pvVar1);
    cVar3 = cVar2;
    if ((iVar9 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  if (cVar3 != '\0') {
    pvVar10 = pvVar1;
    iVar9 = FUN_00509ae0(this,2);
    iVar11 = FUN_00509ae0(this,1);
    iVar9 = FUN_00517ee0(this,local_44,iVar11,iVar9,pvVar10);
    if ((iVar9 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  iVar9 = thunk_FUN_00506e80();
  if ((iVar9 == 0) || (bVar5 = true, !bVar4)) {
    bVar5 = false;
  }
  bVar4 = bVar5;
  if ((iVar9 != 0) &&
     ((bVar4 = FUN_00560d00(this,local_44,pvVar1), CONCAT31(extraout_var_04,bVar4) == 0 ||
      (bVar4 = true, !bVar5)))) {
    bVar4 = false;
  }
LAB_005174c3:
  iVar9 = FUN_00516cd0(param_1);
  if ((iVar9 != 0) && (bVar4)) {
    ExceptionList = local_c;
    return 1;
  }
  ExceptionList = local_c;
  return 0;
}

