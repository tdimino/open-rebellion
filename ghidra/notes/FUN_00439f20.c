
void __fastcall FUN_00439f20(int param_1)

{
  void *this;
  undefined4 *puVar1;
  int iVar2;
  undefined4 *puVar3;
  
  if ((*(uint *)(param_1 + 0x150) & 0x10000000) == 0) {
    this = (void *)(param_1 + 0x154);
    iVar2 = FUN_0048b460(*(int *)(param_1 + 0x14),this);
    if (iVar2 != 0) {
      *(uint *)(param_1 + 0x150) = *(uint *)(param_1 + 0x150) | 0x10000000;
    }
    puVar3 = (undefined4 *)thunk_FUN_005f5080((int)this);
    while (puVar1 = puVar3, puVar1 != (undefined4 *)0x0) {
      puVar3 = (undefined4 *)FUN_005f5c60((int)puVar1);
      if ((((byte)puVar1[7] & 0xf) == 7) || (DAT_006b28d4 != 0)) {
        FUN_0048a590((void *)(*(int *)(param_1 + 0x144) + 0x6c),(int)puVar1);
        FUN_005f54d0(this,puVar1[6]);
        if (puVar1 != (undefined4 *)0x0) {
          (**(code **)*puVar1)(1);
        }
      }
    }
  }
  return;
}

