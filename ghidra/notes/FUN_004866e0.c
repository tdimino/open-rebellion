
void __fastcall FUN_004866e0(int param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = *(int *)(param_1 + 0xc0);
  *(undefined4 *)(iVar1 + 0x188) = 0;
  *(undefined1 *)(iVar1 + 0x170) = 8;
  iVar2 = *(int *)(iVar1 + 0xc4) - *(int *)(iVar1 + 200);
  if (iVar2 < 2) {
    if (iVar2 == 1) {
      *(undefined1 *)(iVar1 + 0x170) = 2;
    }
    else if (iVar2 == -1) {
      *(undefined1 *)(iVar1 + 0x170) = 0x20;
    }
    else if (iVar2 < -1) {
      *(undefined1 *)(iVar1 + 0x170) = 0x40;
    }
  }
  else {
    *(undefined1 *)(iVar1 + 0x170) = 1;
  }
  if (*(char *)(iVar1 + 0x170) == '\b') {
    if (*(int *)(iVar1 + 0xd8) == 0) {
      iVar2 = *(int *)(iVar1 + 0xd4) * 10;
    }
    else {
      iVar2 = (*(int *)(iVar1 + 0xd4) * 100) / *(int *)(iVar1 + 0xd8);
    }
    if (299 < iVar2) {
      *(undefined1 *)(iVar1 + 0x170) = 2;
      return;
    }
    if (199 < iVar2) {
      *(undefined1 *)(iVar1 + 0x170) = 4;
      return;
    }
    if (iVar2 < 0x33) {
      *(undefined1 *)(iVar1 + 0x170) = 0x10;
      return;
    }
    if (iVar2 < 0x22) {
      *(undefined1 *)(iVar1 + 0x170) = 0x20;
    }
  }
  return;
}

