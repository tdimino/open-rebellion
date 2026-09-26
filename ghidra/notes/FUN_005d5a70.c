
undefined4 __cdecl FUN_005d5a70(char *param_1,int param_2,undefined4 *param_3)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined4 uVar4;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006547f8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar2 = FUN_005f35b0(local_18,param_1);
  local_4 = 0;
  for (iVar3 = FUN_005f5060(param_2 + 0x10); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    bVar1 = FUN_005f33f0((void *)(iVar3 + 0x18),(int)pvVar2);
    if (CONCAT31(extraout_var,bVar1) != 0) goto LAB_005d5ace;
  }
  iVar3 = 0;
LAB_005d5ace:
  if (iVar3 == 0) {
    uVar4 = (**(code **)(*(int *)(param_2 + 0xc) + 4))(pvVar2);
    *param_3 = uVar4;
  }
  else {
    *param_3 = *(undefined4 *)(iVar3 + 0x24);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = pvStack_c;
  return 0;
}

