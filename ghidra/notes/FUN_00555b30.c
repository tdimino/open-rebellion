
int FUN_00555b30(int param_1,uint param_2,uint *param_3,uint *param_4,int *param_5)

{
  uint uVar1;
  bool bVar2;
  int *piVar3;
  int *piVar4;
  uint uVar5;
  uint *puVar6;
  uint *puVar7;
  short *psVar8;
  short *psVar9;
  int iVar10;
  int iVar11;
  int *local_24;
  int *local_20;
  uint uStack_1c;
  uint uStack_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  uVar5 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649538;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_5 = 0;
  local_24 = (int *)0x0;
  local_20 = (int *)0x0;
  piVar3 = (int *)FUN_0053efd0(param_3,param_2);
  piVar4 = (int *)FUN_0053efd0(param_4,uVar5);
  if ((piVar4 == (int *)0x0) || (piVar3 == (int *)0x0)) {
    iVar11 = 0;
  }
  else {
    iVar11 = 1;
  }
  if (piVar3 != (int *)0x0) {
    uVar5 = FUN_00555540(piVar3,(int *)&local_24);
    if ((uVar5 == 0) || (iVar11 == 0)) {
      iVar11 = 0;
    }
    else {
      iVar11 = 1;
    }
  }
  if (piVar4 != (int *)0x0) {
    uVar5 = FUN_00555540(piVar4,(int *)&local_20);
    if ((uVar5 == 0) || (iVar11 == 0)) {
      iVar11 = 0;
    }
    else {
      iVar11 = 1;
    }
  }
  if (local_24 == (int *)0x0) {
    ExceptionList = local_c;
    return iVar11;
  }
  if (local_20 == (int *)0x0) {
    ExceptionList = local_c;
    return iVar11;
  }
  puVar6 = FUN_004025b0(local_20,(uint *)&param_3);
  local_4 = 0;
  puVar7 = FUN_004025b0(local_24,&param_2);
  uVar5 = *puVar7;
  uVar1 = *puVar6;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (uVar5 == uVar1) {
    ExceptionList = local_c;
    return iVar11;
  }
  local_14 = 0x90;
  *param_5 = 1;
  local_10 = 0x98;
  local_4 = 1;
  uVar5 = (**(code **)(*local_24 + 4))();
  if ((local_14 <= uVar5) && (uVar5 < local_10)) {
    uStack_1c = 0x90;
    uStack_18 = 0x98;
    local_4._0_1_ = 2;
    uVar5 = (**(code **)(*local_20 + 4))();
    if ((uVar5 < uStack_1c) || (uStack_18 <= uVar5)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    if (bVar2) {
      bVar2 = true;
      goto LAB_00555cc5;
    }
  }
  bVar2 = false;
LAB_00555cc5:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    psVar8 = (short *)FUN_00509620(local_20,&param_2);
    psVar9 = (short *)FUN_00509620(local_24,&param_3);
    iVar10 = FUN_0055d8c0(psVar9,psVar8,param_1);
    *param_5 = iVar10;
  }
  ExceptionList = local_c;
  return iVar11;
}

