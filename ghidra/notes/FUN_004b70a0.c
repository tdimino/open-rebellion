
void __thiscall FUN_004b70a0(int param_1,uint *param_2)

{
  void *pvVar1;
  int iVar2;
  
  *param_2 = *param_2 | 0x4000000;
  pvVar1 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if (pvVar1 != (void *)0x0) {
    *param_2 = *param_2 | 0x8000000;
    iVar2 = FUN_0047aa90(*(void **)(param_1 + 0xc),(undefined4 *)(param_1 + 0x5c));
    if ((iVar2 != 0) && ((*(byte *)(iVar2 + 0x30) & 1) == 0)) {
      *param_2 = *param_2 | 0x20000;
    }
  }
  return;
}

