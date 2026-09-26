
void __fastcall FUN_0048a2a0(int param_1)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  
  if ((*(uint *)(param_1 + 8) & 0x10000000) == 0) {
    piVar2 = (int *)thunk_FUN_005f5060(param_1 + 0x10);
    *(undefined4 *)(param_1 + 0x50) = 0;
    while (piVar1 = piVar2, piVar1 != (int *)0x0) {
      piVar2 = (int *)piVar1[4];
      iVar3 = (**(code **)(*piVar1 + 0x30))();
      if (iVar3 == 0) {
        if ((*(byte *)(piVar1 + 9) & 0x10) != 0) {
          *(uint *)(param_1 + 0x50) = *(uint *)(param_1 + 0x50) | piVar1[0xd];
        }
      }
      else {
        FUN_005f54d0((void *)(param_1 + 0x10),piVar1[6]);
        if ((*(byte *)(piVar1 + 9) & 1) == 0) {
          (**(code **)(*piVar1 + 0x34))();
        }
        if (piVar1 != (int *)0x0) {
          (**(code **)*piVar1)(1);
        }
      }
    }
    if (~*(uint *)(param_1 + 0x50) != 0) {
      FUN_0041db40(~*(uint *)(param_1 + 0x50));
    }
    FUN_0041db20(*(uint *)(param_1 + 0x50));
  }
  return;
}

