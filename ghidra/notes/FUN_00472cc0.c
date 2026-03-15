
void __thiscall FUN_00472cc0(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  undefined4 uVar3;
  int *piVar4;
  uint uVar5;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006328f0;
  local_c = ExceptionList;
  if (*(int *)((int)param_1 + 0x44) == 0) {
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x28) = 1;
  }
  else {
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0x28) = 2;
  }
  *(undefined4 *)((int)this + 0x30) = *(undefined4 *)((int)param_1 + 0x58);
  *(undefined4 *)((int)this + 0x34) = *(undefined4 *)((int)param_1 + 0x60);
  *(undefined4 *)((int)this + 0x38) = *(undefined4 *)((int)param_1 + 0x80);
  *(undefined4 *)((int)this + 0x3c) = *(undefined4 *)((int)param_1 + 0x88);
  uVar3 = FUN_0053b860((int)param_1);
  *(undefined4 *)((int)this + 0x1c) = uVar3;
  uVar3 = FUN_0053b870((int)param_1);
  *(undefined4 *)((int)this + 0x20) = uVar3;
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)param_1 + 0x50);
  local_4 = 0;
  piVar4 = (int *)FUN_00402e40(param_1,&local_10);
  if (*piVar4 == 0x3c000001) {
LAB_00472d9e:
    bVar2 = true;
  }
  else {
    local_4._0_1_ = 2;
    piVar4 = (int *)FUN_00402e40(param_1,&local_14);
    iVar1 = *piVar4;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    if (iVar1 == 0x3c000005) goto LAB_00472d9e;
    bVar2 = false;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    uVar5 = *(uint *)((int)this + 0x18) | 0x1000000;
    goto LAB_00472f97;
  }
  local_4 = 3;
  piVar4 = (int *)FUN_00402e40(param_1,&local_14);
  if (*piVar4 == 0x3c000002) {
LAB_00472e3a:
    bVar2 = true;
  }
  else {
    local_4._0_1_ = 5;
    piVar4 = (int *)FUN_00402e40(param_1,&local_10);
    iVar1 = *piVar4;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    if (iVar1 == 0x3c000006) goto LAB_00472e3a;
    bVar2 = false;
  }
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    uVar5 = *(uint *)((int)this + 0x18) | 0x2000000;
    goto LAB_00472f97;
  }
  local_4 = 6;
  piVar4 = (int *)FUN_00402e40(param_1,&local_14);
  if (*piVar4 == 0x3c000003) {
LAB_00472ed3:
    bVar2 = true;
  }
  else {
    local_4._0_1_ = 8;
    piVar4 = (int *)FUN_00402e40(param_1,&local_10);
    iVar1 = *piVar4;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,7);
    FUN_00619730();
    if (iVar1 == 0x3c000007) goto LAB_00472ed3;
    bVar2 = false;
  }
  local_4 = CONCAT31(local_4._1_3_,6);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    uVar5 = *(uint *)((int)this + 0x18) | 0x4000000;
    goto LAB_00472f97;
  }
  local_4 = 9;
  piVar4 = (int *)FUN_00402e40(param_1,&local_14);
  if (*piVar4 == 0x3c000004) {
LAB_00472f6e:
    bVar2 = true;
  }
  else {
    local_4._0_1_ = 0xb;
    piVar4 = (int *)FUN_00402e40(param_1,&local_10);
    iVar1 = *piVar4;
    FUN_00619730();
    local_4 = CONCAT31(local_4._1_3_,10);
    FUN_00619730();
    if (iVar1 == 0x3c000008) goto LAB_00472f6e;
    bVar2 = false;
  }
  local_4 = CONCAT31(local_4._1_3_,9);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar2) {
    ExceptionList = local_c;
    return;
  }
  uVar5 = *(uint *)((int)this + 0x18) | 0x8000000;
LAB_00472f97:
  *(uint *)((int)this + 0x18) = uVar5;
  ExceptionList = local_c;
  return;
}

