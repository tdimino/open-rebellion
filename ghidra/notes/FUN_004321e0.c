
void __fastcall FUN_004321e0(void *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  
  if (*(int *)((int)param_1 + 0x20) != 0) {
    *(undefined4 *)((int)param_1 + 0x20) = 0;
    FUN_004fcdd0((undefined4 *)((int)param_1 + 0x24));
    *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) & 0x30000000;
    *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) & 0x28000000;
    piVar3 = (int *)((int)param_1 + 0x70);
    for (iVar2 = 0x5a; iVar2 != 0; iVar2 = iVar2 + -1) {
      *piVar3 = 0;
      piVar3 = piVar3 + 1;
    }
    *(undefined4 *)((int)param_1 + 0x144) = 100000;
    piVar3 = (int *)((int)param_1 + 0x48);
    iVar2 = 10;
    do {
      FUN_00433e40((void *)*piVar3);
      iVar1 = *piVar3;
      FUN_00432350(param_1,(int *)(iVar1 + 0x48),(uint *)(iVar1 + 0x28),(uint *)(iVar1 + 0x30),
                   (uint *)(iVar1 + 0x2c));
      piVar3 = piVar3 + 1;
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
    if ((*(uint *)((int)param_1 + 0x2c) & 0x100) == 0) {
      *(undefined4 *)((int)param_1 + 0x144) = 0;
    }
    if (0 < *(int *)((int)param_1 + 0x7c)) {
      *(int *)((int)param_1 + 0x8c) = *(int *)((int)param_1 + 0x8c) / *(int *)((int)param_1 + 0x7c);
    }
    if (*(int *)((int)param_1 + 0x70) == 10) {
      *(uint *)((int)param_1 + 0x28) = *(uint *)((int)param_1 + 0x28) | 0x40000000;
    }
    if (*(int *)((int)param_1 + 0x74) == 10) {
      *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 1;
    }
    if (*(int *)((int)param_1 + 0x78) == 10) {
      *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 2;
    }
    if (0 < *(int *)((int)param_1 + 0x19c)) {
      *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x80;
    }
    if (*(int *)((int)param_1 + 0x30) == 0) {
      if (*(int *)((int)param_1 + 0x38) != 0) {
        *(undefined4 *)((int)param_1 + 0x18c) = 1;
      }
    }
    else {
      *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x4000000;
      if (*(int *)((int)param_1 + 0x38) == 0) {
        *(undefined4 *)((int)param_1 + 0x18c) = 0xffffffff;
        *(uint *)((int)param_1 + 0x2c) = *(uint *)((int)param_1 + 0x2c) | 0x10000;
        return;
      }
    }
  }
  return;
}

