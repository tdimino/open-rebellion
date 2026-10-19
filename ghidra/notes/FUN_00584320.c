
undefined4 FUN_00584320(int *param_1,int *param_2)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  undefined4 local_4;
  
  local_4 = 0;
  if ((param_1 != (int *)0x0) && (param_2 != (int *)0x0)) {
    uVar1 = param_1[6];
    uVar2 = param_2[6];
    uVar3 = (**(code **)(*param_1 + 4))();
    uVar4 = (**(code **)(*param_2 + 4))();
    if ((uVar3 < uVar4) || ((uVar3 == uVar4 && (uVar1 < uVar2)))) {
      local_4 = 1;
    }
  }
  return local_4;
}

