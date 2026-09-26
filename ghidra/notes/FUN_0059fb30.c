
void __fastcall FUN_0059fb30(int param_1)

{
  undefined4 uVar1;
  int *piVar2;
  int iVar3;
  undefined4 *puVar4;
  int *piVar5;
  int local_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  undefined4 uStack_8;
  undefined4 uStack_4;
  
  puVar4 = (undefined4 *)(param_1 + 0x144);
  piVar5 = (int *)(param_1 + 0x114);
  local_14 = 4;
  do {
    for (piVar2 = (int *)(**(code **)(*piVar5 + 0xc))(); piVar2 != (int *)0x0;
        piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
      iVar3 = (**(code **)(*piVar2 + 8))();
      uVar1 = *(undefined4 *)(iVar3 + 0x54);
      iVar3 = (**(code **)(*piVar2 + 8))();
      uStack_10 = *(undefined4 *)(iVar3 + 0x44);
      uStack_c = *(undefined4 *)(iVar3 + 0x48);
      uStack_8 = *(undefined4 *)(iVar3 + 0x4c);
      uStack_4 = *(undefined4 *)(iVar3 + 0x50);
      FUN_005c10d0(piVar2[6],&uStack_10,*puVar4,uVar1);
    }
    piVar5 = piVar5 + 3;
    puVar4 = puVar4 + 1;
    local_14 = local_14 + -1;
  } while (local_14 != 0);
  return;
}

