
void __cdecl FUN_005d38d0(int param_1,int param_2,undefined4 *param_3,undefined4 param_4)

{
  int *piVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 uVar4;
  undefined4 unaff_EBX;
  undefined4 unaff_EBP;
  undefined4 *puVar5;
  int iVar6;
  
  FUN_005fefd0(6);
  uVar2 = param_4;
  iVar6 = 0;
  puVar5 = param_3;
  if (0 < param_2) {
    do {
      param_3 = (undefined4 *)(iVar6 + param_1);
      piVar1 = (int *)(DAT_006bcbd4 + 0x168);
      for (iVar3 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
        if (*(short *)(iVar3 + 0x18) == (short)param_3) goto LAB_005d392a;
      }
      iVar3 = 0;
LAB_005d392a:
      if (iVar3 == 0) {
        uVar4 = (**(code **)(*piVar1 + 4))(&param_3);
        *puVar5 = uVar4;
      }
      else {
        *puVar5 = *(undefined4 *)(iVar3 + 0x1c);
      }
      (**(code **)(*(int *)*puVar5 + 0x3c))((int *)*puVar5,0x100);
      (**(code **)(*(int *)*puVar5 + 0x40))((int *)*puVar5,1);
      (**(code **)(*(int *)*puVar5 + 0x50))((int *)*puVar5,1);
      (**(code **)(*(int *)*puVar5 + 0x44))((int *)*puVar5,uVar2,uVar2);
      (**(code **)(*(int *)*puVar5 + 0x48))((int *)*puVar5,unaff_EBX,unaff_EBP);
      iVar6 = iVar6 + 1;
      puVar5 = puVar5 + 1;
    } while (iVar6 < param_2);
  }
  return;
}

