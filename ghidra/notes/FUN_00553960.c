
int __cdecl FUN_00553960(undefined4 param_1,int param_2,void *param_3)

{
  int *piVar1;
  int iVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649088;
  local_c = ExceptionList;
  iVar2 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = (int *)FUN_0041c210(param_2);
  while (((piVar1 != (int *)0x0 && (iVar2 != 0)) && (*(int *)((int)param_3 + 4) == -1))) {
    iVar2 = (**(code **)(*piVar1 + 0x1c))(param_3);
    piVar1 = (int *)piVar1[4];
  }
  ExceptionList = local_c;
  return iVar2;
}

