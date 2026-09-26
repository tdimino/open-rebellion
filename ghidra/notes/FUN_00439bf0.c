
void __fastcall FUN_00439bf0(int param_1)

{
  int iVar1;
  
  *(undefined4 *)(param_1 + 0x188) = 0;
  *(undefined1 *)(param_1 + 0x170) = 8;
  iVar1 = *(int *)(param_1 + 0xc4) - *(int *)(param_1 + 200);
  if (iVar1 < 2) {
    if (iVar1 == 1) {
      *(undefined1 *)(param_1 + 0x170) = 2;
    }
    else if (iVar1 == -1) {
      *(undefined1 *)(param_1 + 0x170) = 0x20;
    }
    else if (iVar1 < -1) {
      *(undefined1 *)(param_1 + 0x170) = 0x40;
    }
  }
  else {
    *(undefined1 *)(param_1 + 0x170) = 1;
  }
  if (*(char *)(param_1 + 0x170) == '\b') {
    if (*(int *)(param_1 + 0xd8) == 0) {
      iVar1 = *(int *)(param_1 + 0xd4) * 10;
    }
    else {
      iVar1 = (*(int *)(param_1 + 0xd4) * 100) / *(int *)(param_1 + 0xd8);
    }
    if (299 < iVar1) {
      *(undefined1 *)(param_1 + 0x170) = 2;
      return;
    }
    if (199 < iVar1) {
      *(undefined1 *)(param_1 + 0x170) = 4;
      return;
    }
    if (iVar1 < 0x33) {
      *(undefined1 *)(param_1 + 0x170) = 0x10;
      return;
    }
    if (iVar1 < 0x22) {
      *(undefined1 *)(param_1 + 0x170) = 0x20;
    }
  }
  return;
}

