
void __fastcall FUN_00406ac0(undefined4 *param_1)

{
  undefined4 uVar1;
  int *piVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006298e8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00658600;
  local_4 = 0;
  piVar2 = param_1 + 0x31;
  iVar3 = 6;
  do {
    if ((undefined4 *)*piVar2 != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)*piVar2)(1);
    }
    piVar2 = piVar2 + 1;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  if ((undefined4 *)param_1[0x37] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x37])(1);
  }
  if ((undefined4 *)param_1[0x38] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x38])(1);
  }
  FUN_00408db0((int)param_1);
  if (((int)param_1[0x2a] < 0x14) || (0x17 < (int)param_1[0x2a])) {
    FUN_00401240();
  }
  else {
    uVar1 = FUN_0041cdb0();
    FUN_0041d310(uVar1);
  }
  local_4 = 0xffffffff;
  FUN_005ff250(param_1);
  ExceptionList = local_c;
  return;
}

