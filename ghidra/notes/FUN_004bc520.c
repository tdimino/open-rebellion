
int * __fastcall FUN_004bc520(void *param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  
  piVar4 = (int *)0x0;
  FUN_0041b600(*(void **)((int)param_1 + 0x44),*(uint *)((int)param_1 + 0x18));
  uVar2 = *(uint *)((int)param_1 + 0x2c) >> 0x18;
  if ((uVar2 < 0x30) || (0x3f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940(*(void **)((int)param_1 + 0x40),(uint *)((int)param_1 + 0x2c));
    if ((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x28) == *(int *)((int)param_1 + 0x18)))
    {
      *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x800000;
      *(undefined4 *)((int)pvVar3 + 0x2c) = 0;
      if (((*(uint *)((int)pvVar3 + 0x30) & 0x7000000) != 0) &&
         ((*(uint *)((int)pvVar3 + 0x30) & 0xa0) == 0)) {
        piVar4 = FUN_004bbf50(param_1,0x260);
      }
    }
  }
  FUN_004ece80((undefined4 *)((int)param_1 + 0x28));
  return piVar4;
}

