
/* WARNING: Restarted to delay deadcode elimination for space: stack */

uint __thiscall FUN_005c2d20(void *this,int *param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  int *unaff_EBP;
  int *piVar6;
  int unaff_ESI;
  int *unaff_EDI;
  int *piStack_40;
  int local_28 [3];
  undefined4 local_1c;
  void *local_18;
  int iStack_10;
  
  uVar5 = 0;
  local_18 = this;
  local_1c = FUN_00595bf0();
  if ((((0 < (int)param_1) && (0 < param_2)) && ((int)param_1 < *(int *)((int)this + 0x30))) &&
     ((param_2 < *(int *)((int)this + 0x34) && (*(int *)((int)this + 0x638) != 0)))) {
    piStack_40 = local_28;
    (**(code **)(**(int **)(*(int *)((int)this + 0x638) + 4) + 0x70))();
    iVar1 = (**(code **)(*unaff_EDI + 0xc))();
    piVar6 = (int *)0x0;
    iStack_10 = iVar1;
    if (0 < iVar1) {
      do {
        (**(code **)(*unaff_EBP + 0x10))(unaff_EBP);
        uVar2 = (**(code **)(*param_1 + 0x1c))(param_1);
        if (((uint)param_1 & uVar2) != 0) {
          iVar3 = (*(code *)piStack_40[3])(&piStack_40);
          while (iVar3 = iVar3 + -1, -1 < iVar3) {
            (*(code *)piStack_40[4])(&piStack_40,iVar3,&stack0xffffffb0);
            uVar2 = (**(code **)(unaff_ESI + 0x1c))(&stack0xffffffcc);
            (**(code **)(*piVar6 + 8))(piVar6);
            iVar1 = local_28[0];
            if ((uVar5 == 0) &&
               (iVar4 = FUN_005f5500((void *)(param_2 + 0x5d8),uVar2), iVar1 = local_28[0],
               *(int *)(iVar4 + 0x34) != 0)) {
              uVar5 = uVar2;
            }
          }
        }
        (*(code *)piStack_40[2])(&piStack_40);
        (**(code **)(*unaff_EDI + 8))(unaff_EDI);
        piVar6 = (int *)((int)piVar6 + 1);
      } while ((int)piVar6 < iVar1);
    }
    (**(code **)(*unaff_EBP + 8))(unaff_EBP);
  }
  return uVar5;
}

