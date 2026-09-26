
void __fastcall FUN_0059e760(void *param_1)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  
  if (DAT_006bc464 != (int *)0x0) {
    uVar1 = (**(code **)(*DAT_006bc464 + 8))();
    switch(uVar1) {
    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
      iVar2 = (**(code **)(*DAT_006bc464 + 8))();
      FUN_0059fc50(param_1,iVar2 - 1);
      break;
    case 9:
    case 10:
    case 0xb:
    case 0xc:
      FUN_005c0e70();
      iVar3 = 1;
      iVar2 = (**(code **)(*DAT_006bc464 + 8))();
      FUN_005c1040(iVar2 + -9,iVar3);
    }
  }
  iVar2 = FUN_005aaf90();
  if (*(int *)(iVar2 + 0x8c8) == 0) {
    (**(code **)(*DAT_006bc464 + 0x3c))(1);
  }
  iVar2 = FUN_005aaf90();
  FUN_005c0f80((uint)(*(int *)(iVar2 + 0x8c8) != 0));
  return;
}

