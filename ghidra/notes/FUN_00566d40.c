
int __thiscall FUN_00566d40(int param_1,undefined4 param_2)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  
  iVar2 = FUN_005081f0(param_2);
  if (iVar2 != 0) {
    iVar3 = FUN_005596f0();
    uVar5 = 0;
    if ((iVar3 != 0) && (*(int *)(param_1 + 0x18) != 0x109)) {
      uVar1 = *(uint *)(param_1 + 0x24);
      uVar5 = FUN_005092f0();
      uVar5 = FUN_00559700(uVar1 >> 6 & 3,uVar5);
    }
    FUN_0050a3c0(iVar3,param_2);
    if (iVar3 != 0) {
      FUN_00509c30(uVar5,param_2);
    }
    uVar5 = param_2;
    uVar4 = FUN_00559670(param_2);
    FUN_00509cc0(uVar4,uVar5);
    uVar5 = param_2;
    uVar4 = FUN_005596a0(*(undefined4 *)(param_1 + 0x5c),param_2);
    FUN_00509dc0(uVar4,uVar5);
    FUN_0050a510(0,param_2);
  }
  return iVar2;
}

