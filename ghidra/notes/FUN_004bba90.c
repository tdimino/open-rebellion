
undefined4 __fastcall FUN_004bba90(int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  undefined4 uVar4;
  
  uVar4 = 1;
  uVar2 = *(uint *)(param_1 + 0x2c) >> 0x18;
  if ((uVar2 < 0x30) || (0x3f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940(*(void **)(param_1 + 0x40),(uint *)(param_1 + 0x2c));
    if ((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x30) & 0xa0) != 0)) {
      uVar4 = 0;
    }
  }
  return uVar4;
}

