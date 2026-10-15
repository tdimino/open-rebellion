
bool __thiscall FUN_0054a9c0(void *param_1,uint param_2,void *param_3)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  bool bVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  pvVar1 = param_3;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00647dc8;
  pvStack_c = ExceptionList;
  bVar4 = true;
  if (param_2 != 0) {
    ExceptionList = &pvStack_c;
    FUN_00543fc0(param_1,0,param_3);
    iVar2 = thunk_FUN_00506e60();
    bVar4 = iVar2 != 0;
    if (iVar2 != 0) {
      param_2 = 0x33000243;
      uStack_4 = 0;
      uVar3 = FUN_00556c70(&param_2,1,(int)pvVar1);
      if ((uVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

