
void __thiscall FUN_0045d360(void *this,void *param_1)

{
  int iVar1;
  int *piVar2;
  uint *puVar3;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630c58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 0;
  FUN_004ece90(&local_14,param_1);
  FUN_005f4d90(param_1,&local_10);
  puVar3 = &local_14;
  iVar1 = FUN_00401060();
  piVar2 = FUN_004f3970(iVar1,puVar3);
  *(int **)((int)this + 0x144) = piVar2;
  *(undefined4 *)((int)this + 0x24) = local_10;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

