
void __fastcall FUN_005aee90(int param_1)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int iVar7;
  
  piVar2 = (int *)(**(code **)(*(int *)(param_1 + 0x4c) + 0x10))();
  do {
    if (piVar2 == (int *)0x0) {
      return;
    }
    piVar3 = (int *)(**(code **)(*piVar2 + 0x10))();
    iVar4 = (**(code **)(*piVar3 + 0x20))();
    if ((iVar4 == 1) &&
       ((((iVar4 = piVar3[0x5c], iVar4 == 2 || (iVar4 == 1)) || (iVar4 == 5)) || (iVar4 == 9)))) {
      bVar1 = false;
      for (piVar5 = (int *)(**(code **)(*(int *)(param_1 + 0x4c) + 0x10))(); piVar5 != (int *)0x0;
          piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
        if (bVar1) goto LAB_005aef9b;
        piVar6 = (int *)(**(code **)(*piVar5 + 0x10))();
        iVar4 = (**(code **)(*piVar6 + 0x20))();
        bVar1 = false;
        if ((iVar4 == 0) &&
           (iVar4 = (**(code **)(*piVar5 + 0x10))(), *(int *)(iVar4 + 0x168) == piVar3[0x5a])) {
          iVar4 = (**(code **)(*piVar5 + 0x10))();
          if (((*(int *)(iVar4 + 0x170) == 2) || (*(int *)(iVar4 + 0x170) == 9)) &&
             (iVar4 = FUN_005b84a0((void *)(iVar4 + 0x418),(int)piVar3), iVar4 == 1)) {
            bVar1 = true;
          }
        }
      }
      if (!bVar1) {
        iVar4 = FUN_00596bd0();
        iVar4 = FUN_005a66a0(iVar4);
        if (((iVar4 != 0) &&
            ((iVar7 = FUN_005ba420(iVar4), iVar7 != 0 || (*(int *)(iVar4 + 0x78) != 0)))) &&
           (piVar3[0x5a] == 1)) {
          FUN_005b84a0((void *)(iVar4 + 0x1c),(int)piVar3);
        }
      }
    }
LAB_005aef9b:
    piVar2 = (int *)(**(code **)(*piVar2 + 8))();
  } while( true );
}

