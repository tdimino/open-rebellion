
int * __thiscall FUN_00584290(void *this,uint *param_1)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  int *piVar4;
  int *piVar5;
  
  bVar1 = false;
  piVar4 = *(int **)((int)this + 4);
  piVar5 = (int *)0x0;
  while ((piVar4 != (int *)0x0 && (!bVar1))) {
    uVar2 = (**(code **)(*piVar4 + 4))();
    uVar3 = *param_1;
    bVar1 = false;
    piVar5 = piVar4;
    if ((uVar2 < uVar3) || ((uVar3 == uVar2 && (piVar4[6] == 0)))) {
      piVar4 = (int *)piVar4[2];
    }
    else if ((uVar3 < uVar2) || ((uVar3 == uVar2 && (1 < (uint)piVar4[6])))) {
      piVar4 = (int *)piVar4[1];
    }
    else {
      bVar1 = true;
    }
  }
  if ((piVar4 == (int *)0x0) && (piVar5 != (int *)0x0)) {
    uVar3 = (**(code **)(*piVar5 + 4))();
    if (uVar3 < *param_1) {
      piVar5 = (int *)piVar5[4];
    }
    if (piVar5 != (int *)0x0) {
      (**(code **)(*piVar5 + 4))();
      uVar3 = (**(code **)(*piVar5 + 4))();
      if ((*param_1 <= uVar3) && (uVar3 < param_1[1])) {
        piVar4 = piVar5;
      }
    }
  }
  return piVar4;
}

