
int __thiscall FUN_0052a240(void *this,int param_1,void *param_2)

{
  uint *puVar1;
  int iVar2;
  int iVar3;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644908;
  local_c = ExceptionList;
  iVar3 = 1;
  ExceptionList = &local_c;
  local_10 = this;
  puVar1 = (uint *)FUN_0040d760(this,&local_10);
  local_4 = 0;
  iVar2 = FUN_004ece60(puVar1);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (iVar2 != 0) {
    iVar3 = FUN_00529980(this,param_1,param_2);
  }
  ExceptionList = local_c;
  return iVar3;
}

