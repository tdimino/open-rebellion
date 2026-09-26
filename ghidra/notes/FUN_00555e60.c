
bool FUN_00555e60(void *param_1,void *param_2,int *param_3)

{
  void *pvVar1;
  int *piVar2;
  bool bVar3;
  undefined3 extraout_var;
  int iVar4;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_2;
  puStack_8 = &LAB_00649578;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar2 = param_3;
  *param_3 = 0;
  param_2 = (void *)0x0;
  bVar3 = FUN_00529360(param_1,pvVar1,(int *)&param_2);
  if (*(int *)((int)pvVar1 + 4) == -1) {
    iVar4 = FUN_004fd340();
    *piVar2 = iVar4 + (int)param_2;
  }
  ExceptionList = local_c;
  return CONCAT31(extraout_var,bVar3) != 0;
}

