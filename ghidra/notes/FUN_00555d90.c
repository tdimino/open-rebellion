
bool FUN_00555d90(void *param_1,uint param_2,uint *param_3,uint *param_4,int *param_5)

{
  void *pvVar1;
  int *piVar2;
  uint *puVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_5;
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649558;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_5 = 0;
  iVar4 = 0;
  puVar3 = (uint *)FUN_00402e40(param_1,(uint *)&param_1);
  local_4 = 0;
  if ((*puVar3 >> 0x18 < 0x14) || (param_5 = (int *)0x1, 0x1f < *puVar3 >> 0x18)) {
    param_5 = (int *)0x0;
  }
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_5 != (int *)0x0) {
    iVar4 = *(int *)((int)pvVar1 + 0x6c);
  }
  if (iVar4 == 0) {
    iVar4 = DAT_006b9050;
  }
  iVar4 = FUN_00555b30(iVar4,param_2,param_3,param_4,piVar2);
  ExceptionList = local_c;
  return iVar4 != 0;
}

