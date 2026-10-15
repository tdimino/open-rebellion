
int __thiscall FUN_0058e6b0(int param_1,undefined4 param_2)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  undefined4 uStack_24;
  int *piStack_20;
  int iStack_c;
  int iStack_8;
  undefined1 *puStack_4;
  
  piStack_20 = (int *)0x58e6c2;
  (**(code **)(*(int *)(param_1 + 0x10) + 4))();
  uVar1 = param_2;
  piStack_20 = (int *)param_2;
  uStack_24 = 0x58e6ce;
  iVar2 = FUN_00583530();
  if (iVar2 != 0) {
    piStack_20 = &iStack_c;
    uStack_24 = uVar1;
    FUN_005f4d90();
    piStack_20 = &param_2;
    uStack_24 = uVar1;
    FUN_005f4d90();
    piStack_20 = &iStack_8;
    uStack_24 = uVar1;
    FUN_005f4d90();
    *(undefined4 *)(param_1 + 8) = param_2;
    *(int *)(param_1 + 0xc) = iStack_8;
    do {
      if (iStack_c == 0) {
        return iVar2;
      }
      puStack_4 = (undefined1 *)&uStack_24;
      iStack_c = iStack_c + -1;
      FUN_00520580(param_1 + 8);
      piVar3 = (int *)FUN_00584ac0();
      if (piVar3 != (int *)0x0) {
        piStack_20 = (int *)uVar1;
        uStack_24 = 0x58e747;
        iVar2 = (**(code **)(*piVar3 + 8))();
        if (iVar2 == 0) {
          piStack_20 = (int *)0x1;
          uStack_24 = 0x58e75f;
          (**(code **)*piVar3)();
        }
        else {
          uStack_24 = 0x58e755;
          piStack_20 = piVar3;
          FUN_0058ead0();
        }
      }
    } while (iVar2 != 0);
  }
  return 0;
}

