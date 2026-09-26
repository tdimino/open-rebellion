
void __fastcall FUN_0048e3c0(int param_1)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  
  iVar2 = FUN_004f3e50(*(int *)(param_1 + 0x20));
  if (iVar2 == 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
    return;
  }
  FUN_0060b9d0((void *)(param_1 + 0x44),0x71e0,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0
               ,(undefined4 *)0x0);
  if ((*(byte *)(iVar2 + 0xac) & 1) == 0) {
    uVar3 = 0x71e1;
  }
  else {
    uVar3 = 0x71e2;
  }
  FUN_0060b9d0((void *)(param_1 + 0x50),uVar3,(undefined4 *)0x0,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  if ((*(byte *)(iVar2 + 0xb0) & 8) != 0) {
    *(undefined2 *)(param_1 + 0x2e) = 0x428;
    uVar1 = FUN_004c4990(&DAT_0000012f);
    *(ushort *)(param_1 + 0x32) = uVar1;
    *(undefined2 *)(param_1 + 0x30) = 0x473;
    return;
  }
  *(undefined2 *)(param_1 + 0x2e) = 0x427;
  uVar1 = FUN_004c4990(&DAT_0000011c);
  *(ushort *)(param_1 + 0x32) = uVar1;
  *(undefined2 *)(param_1 + 0x30) = 0x472;
  return;
}

