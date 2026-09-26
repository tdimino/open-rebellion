
undefined4 __cdecl FUN_00584ac0(int param_1,int param_2)

{
  int iVar1;
  undefined4 uVar2;
  int *piVar3;
  code *pcVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f3b8;
  pvStack_c = ExceptionList;
  uVar2 = 0;
  pcVar4 = (code *)0x0;
  local_4 = 0;
  iVar1 = 0;
  piVar3 = &DAT_006bbb54;
  ExceptionList = &pvStack_c;
  do {
    if (DAT_006bbb48 <= iVar1) break;
    if ((piVar3[-1] == param_1) && (*piVar3 == param_2)) {
      pcVar4 = (code *)piVar3[1];
    }
    FUN_00619730();
    iVar1 = iVar1 + 1;
    piVar3 = piVar3 + 3;
  } while (pcVar4 == (code *)0x0);
  if (pcVar4 != (code *)0x0) {
    uVar2 = (*pcVar4)();
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar2;
}

