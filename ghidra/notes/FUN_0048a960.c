
bool __fastcall FUN_0048a960(int param_1)

{
  int iVar1;
  undefined4 *puVar2;
  int *piVar3;
  bool bVar4;
  
  bVar4 = true;
  if ((*(byte *)(param_1 + 0x14) & 4) == 0) {
    iVar1 = thunk_FUN_005f5060();
    if (iVar1 != 0) {
      do {
        if (*(uint *)(param_1 + 0x1c) < *(uint *)(iVar1 + 0x18)) break;
        puVar2 = (undefined4 *)FUN_00537030(*(uint *)(iVar1 + 0x18));
        if (puVar2 != (undefined4 *)0x0) {
          (**(code **)*puVar2)(1);
        }
        iVar1 = thunk_FUN_005f5060();
      } while (iVar1 != 0);
      if (iVar1 != 0) {
        *(undefined4 *)(param_1 + 0x1c) = *(undefined4 *)(iVar1 + 0x18);
        piVar3 = (int *)FUN_00537030(*(undefined4 *)(iVar1 + 0x18));
        iVar1 = (**(code **)(*piVar3 + 0xc))();
        bVar4 = iVar1 == 1;
        if (bVar4) {
          *(uint *)(param_1 + 0x14) = *(uint *)(param_1 + 0x14) | 4;
        }
        FUN_0041db80(piVar3);
      }
    }
  }
  return bVar4;
}

