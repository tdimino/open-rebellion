
int __thiscall FUN_0052d540(void *param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  int *piVar2;
  undefined1 auStack_14 [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00644f68;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_0055b0c0(param_1,param_2,param_3);
  if (iVar1 != 0) {
    piVar2 = (int *)FUN_0051c900(auStack_14,0x32,0);
    uStack_4 = 0;
    FUN_0055a6e0(param_1,piVar2,param_3);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return iVar1;
}

