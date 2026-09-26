
void __fastcall FUN_004d0480(int param_1)

{
  ushort uVar1;
  bool bVar2;
  bool bVar3;
  ushort uVar4;
  int iVar5;
  
  bVar2 = false;
  bVar3 = false;
  if (*(short *)(param_1 + 0x40) == 0) {
    *(undefined2 *)(param_1 + 0x40) = 0x5a;
  }
  uVar1 = *(ushort *)(param_1 + 0x40);
  uVar4 = uVar1;
  do {
    if (bVar2) {
      return;
    }
    iVar5 = *(int *)(*(int *)(param_1 + 0x2c) + 0x2fc);
    if (iVar5 < 8) {
      iVar5 = 8;
    }
    if (*(int *)(*(int *)(param_1 + 0x2c) + 0x184 + (uint)uVar4 * 4) < iVar5) {
      *(undefined4 *)(param_1 + 0x48) = 1;
    }
    else {
      *(undefined4 *)(param_1 + 0x48) = 0;
    }
    switch((uint)uVar4) {
    case 0x5a:
      *(undefined4 *)(param_1 + 0x50) = 0x1000000;
      *(undefined4 *)(param_1 + 0x54) = 1;
      *(undefined2 *)(param_1 + 0x40) = 0x5b;
      break;
    case 0x5b:
      *(undefined4 *)(param_1 + 0x50) = 0x2000000;
      *(undefined4 *)(param_1 + 0x54) = 1;
      *(undefined2 *)(param_1 + 0x40) = 0x5c;
      break;
    case 0x5c:
      *(undefined4 *)(param_1 + 0x50) = 0x4000000;
      *(undefined4 *)(param_1 + 0x54) = 1;
      *(undefined2 *)(param_1 + 0x40) = 0x5d;
      break;
    case 0x5d:
      *(undefined4 *)(param_1 + 0x50) = 0x8000000;
      *(undefined4 *)(param_1 + 0x54) = 1;
      *(undefined2 *)(param_1 + 0x40) = 0x5a;
    }
    if (0 < *(int *)(param_1 + 0x48)) {
      bVar3 = true;
    }
    uVar4 = *(ushort *)(param_1 + 0x40);
    if (uVar1 == uVar4) {
      bVar2 = true;
    }
  } while (!bVar3);
  return;
}

