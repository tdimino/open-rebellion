
undefined4 __thiscall FUN_004c6770(void *this,int param_1)

{
  uint *puVar1;
  int iVar2;
  undefined4 uVar3;
  int iVar4;
  int *piVar5;
  undefined4 *puVar6;
  
  uVar3 = 0;
  if (*(int *)((int)this + 0x1c) != 0) {
    puVar1 = (uint *)((int)this + 0x24);
    *(undefined4 *)((int)this + 0x1c) = 0;
    FUN_004fcdd0(puVar1);
    puVar6 = (undefined4 *)((int)this + 0x60);
    for (iVar4 = 0x1a; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar6 = 0;
      puVar6 = puVar6 + 1;
    }
    piVar5 = (int *)((int)this + 0x38);
    iVar4 = 10;
    do {
      FUN_004ebe40((void *)*piVar5,param_1);
      iVar2 = *piVar5;
      FUN_004c6830(this,(int *)(iVar2 + 0x40),(uint *)(iVar2 + 0x24),(uint *)(iVar2 + 0x28));
      piVar5 = piVar5 + 1;
      iVar4 = iVar4 + -1;
    } while (iVar4 != 0);
    if (*(int *)((int)this + 0x60) == 10) {
      *puVar1 = *puVar1 | 1;
    }
    if (*(int *)((int)this + 100) == 10) {
      *puVar1 = *puVar1 | 2;
    }
    uVar3 = 1;
  }
  return uVar3;
}

