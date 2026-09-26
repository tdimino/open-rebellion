
void __fastcall FUN_0048db30(int param_1)

{
  bool bVar1;
  uint uVar2;
  
  uVar2 = *(uint *)(param_1 + 0x5c) >> 0x18;
  if ((uVar2 < 0x30) || (0x37 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    switch(*(uint *)(param_1 + 0x5c) & 0xffffff) {
    case 0x240:
      *(undefined4 *)(param_1 + 0x28) = 0x17;
      return;
    case 0x241:
      *(undefined4 *)(param_1 + 0x28) = 0x18;
      return;
    case 0x242:
      *(undefined4 *)(param_1 + 0x28) = 0x16;
      return;
    case 0x243:
      *(undefined4 *)(param_1 + 0x28) = 0x19;
      return;
    case 0x280:
      *(undefined4 *)(param_1 + 0x28) = 0x1a;
      return;
    case 0x281:
      *(undefined4 *)(param_1 + 0x28) = 0x1b;
      return;
    }
  }
  else {
    *(undefined4 *)(param_1 + 0x28) = 0x15;
  }
  return;
}

