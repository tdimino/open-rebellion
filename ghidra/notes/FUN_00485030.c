
void __fastcall FUN_00485030(int param_1)

{
  uint uVar1;
  int *piVar2;
  undefined4 uVar3;
  
  piVar2 = (int *)FUN_00485460();
  FUN_00597140();
  if (piVar2 == (int *)0x0) {
    return;
  }
  uVar3 = (**(code **)(*piVar2 + 0xc))();
  switch(uVar3) {
  case 2:
    if (piVar2[8] != *(int *)(param_1 + 0x24)) {
      FUN_0041cf00(piVar2[0xb]);
      (**(code **)*piVar2)(1);
      return;
    }
    break;
  default:
    FUN_00485440(piVar2);
    return;
  case 4:
    FUN_004be700((void *)(param_1 + 0x58),(int)piVar2);
    (**(code **)*piVar2)(1);
    return;
  case 0x12:
    FUN_004212a0((int)piVar2);
    (**(code **)*piVar2)(1);
    return;
  case 0x15:
    if (piVar2[8] != 0) {
      uVar1 = piVar2[9];
      *(uint *)(param_1 + 0x48) = uVar1;
      FUN_0041d2f0(uVar1);
      (**(code **)*piVar2)(1);
      return;
    }
    FUN_0041d310(piVar2[10]);
  }
  (**(code **)*piVar2)(1);
  return;
}

