
void __fastcall FUN_005df560(int param_1)

{
  void *this;
  uint *puVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  
  puVar1 = (uint *)FUN_005ff440(param_1);
  iVar2 = FUN_005df660(param_1);
  piVar5 = (int *)(param_1 + 0x9c);
  iVar4 = 4;
  do {
    if (*piVar5 != 0) {
      iVar3 = FUN_00602510(*piVar5);
      if (iVar3 != iVar2) {
        FUN_006030f0((void *)*piVar5,4);
      }
    }
    piVar5 = piVar5 + 1;
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  iVar4 = iVar2 + -0xdd03;
  if (iVar2 == 0) {
    iVar4 = 4;
  }
  if ((puVar1 != (uint *)0x0) &&
     (this = *(void **)(param_1 + 0xac + iVar4 * 4), this != (void *)0x0)) {
    FUN_005fcc30(this,puVar1,0xe,0xe,0,0,(undefined4 *)0x7c,0x6a);
    InvalidateRect(*(HWND *)(param_1 + 0x18),(RECT *)0x0,0);
  }
  return;
}

