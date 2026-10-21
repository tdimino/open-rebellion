
int __thiscall FUN_00567110(void *param_1,undefined4 param_2,undefined4 param_3,void *param_4)

{
  int iVar1;
  void *pvVar2;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064b178;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_005109f0(param_1,param_2,param_3,param_4);
  if (iVar1 != 0) {
    FUN_004fd450(&uStack_28,(int)param_4);
    uStack_4 = 0;
    uStack_24 = 1;
    uStack_20 = 3;
    pvVar2 = (void *)FUN_00539fd0(param_1,1);
    FUN_0050f390(pvVar2,&uStack_28);
    pvVar2 = (void *)FUN_00539fd0(param_1,2);
    FUN_0050f390(pvVar2,&uStack_28);
    uStack_4 = 0xffffffff;
    FUN_004fd4d0(&uStack_28);
  }
  ExceptionList = pvStack_c;
  return iVar1;
}

