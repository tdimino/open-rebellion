
undefined4 __fastcall FUN_004b8b30(int param_1)

{
  void *pvVar1;
  undefined4 uVar2;
  
  uVar2 = 0;
  pvVar1 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if ((pvVar1 != (void *)0x0) && (1 < *(int *)(*(int *)(param_1 + 0x10) + 0xcc))) {
    uVar2 = 1;
  }
  return uVar2;
}

