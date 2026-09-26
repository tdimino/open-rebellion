
void __thiscall FUN_005ce6c0(void *this,void *param_1)

{
  int *this_00;
  int *piVar1;
  int iVar2;
  int iVar3;
  int iStack_18;
  int iStack_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006544ab;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*(int *)this + 4))();
  iStack_18 = 0;
  FUN_005f4d90(param_1,&iStack_18);
  if (iStack_18 != 0) {
    iStack_14 = iStack_18;
    do {
      puStack_10 = (undefined4 *)FUN_00618b70(0x220);
      uStack_4 = 0;
      if (puStack_10 == (undefined4 *)0x0) {
        this_00 = (int *)0x0;
      }
      else {
        this_00 = FUN_005ce2b0(puStack_10);
      }
      uStack_4 = 0xffffffff;
      FUN_005ce380(this_00,param_1);
      for (piVar1 = (int *)thunk_FUN_005f5060((int)this); piVar1 != (int *)0x0;
          piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
        iVar2 = (**(code **)(*piVar1 + 0x1c))();
        iVar3 = (**(code **)(*this_00 + 0x1c))();
        if (iVar2 == iVar3) goto LAB_005ce781;
      }
      FUN_005f5440(this,this_00);
LAB_005ce781:
      iStack_14 = iStack_14 + -1;
    } while (iStack_14 != 0);
  }
  ExceptionList = pvStack_c;
  return;
}

