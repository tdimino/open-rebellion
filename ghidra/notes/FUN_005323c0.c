
int __thiscall FUN_005323c0(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  int local_24 [2];
  undefined1 local_1c [8];
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645638;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004292e0(this,local_14);
  iVar4 = *piVar2;
  local_4 = 0;
  piVar2 = FUN_00429300(this,local_1c);
  iVar5 = *piVar2;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (iVar4 < iVar5) {
    piVar2 = FUN_004292e0(this,local_14);
    iVar4 = *piVar2;
  }
  else {
    piVar2 = FUN_00429300(this,local_14);
    iVar4 = *piVar2;
  }
  FUN_00619730();
  pvVar3 = FUN_004292e0(this,local_1c);
  iVar5 = *(int *)((int)pvVar3 + 4);
  local_4 = 1;
  pvVar3 = FUN_00429300(this,local_14);
  iVar1 = *(int *)((int)pvVar3 + 4);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (iVar1 < iVar5) {
    pvVar3 = FUN_004292e0(this,local_14);
    iVar5 = *(int *)((int)pvVar3 + 4);
  }
  else {
    pvVar3 = FUN_00429300(this,local_14);
    iVar5 = *(int *)((int)pvVar3 + 4);
  }
  FUN_00619730();
  if (iVar4 < iVar5) {
    iVar5 = iVar4;
  }
  FUN_0051c900(local_24,iVar4,iVar5);
  local_4 = 2;
  iVar4 = FUN_0052ec90(this,local_24,param_3);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar4;
}

