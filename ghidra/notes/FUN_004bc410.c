
void __thiscall FUN_004bc410(int param_1,undefined4 param_2)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  
  uVar2 = *(uint *)(param_1 + 0x2c) >> 0x18;
  *(undefined4 *)(param_1 + 0x4c) = param_2;
  if ((uVar2 < 0x30) || (0x3f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940(*(void **)(param_1 + 0x40),(uint *)(param_1 + 0x2c));
    if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x28) == *(int *)(param_1 + 0x18))) {
      *(undefined4 *)((int)pvVar3 + 0x2c) = param_2;
    }
  }
  return;
}

