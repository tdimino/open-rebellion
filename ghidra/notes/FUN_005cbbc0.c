
void __fastcall FUN_005cbbc0(int *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  void *pvVar7;
  float *pfVar8;
  float unaff_EBX;
  float unaff_EDI;
  int *piVar9;
  undefined1 auStack_10 [16];
  
  iVar4 = (**(code **)(*param_1 + 4))();
  if (iVar4 != 4) {
    for (piVar5 = (int *)(**(code **)(param_1[7] + 0x10))(); piVar5 != (int *)0x0;
        piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
      piVar6 = (int *)(**(code **)(*piVar5 + 0x10))();
      piVar9 = piVar6;
      pvVar7 = (void *)FUN_005aaf90();
      FUN_005a93b0(pvVar7,piVar9);
      fVar3 = (float)piVar6[0xc];
      pfVar8 = (float *)(**(code **)(*(int *)param_1[0x15] + 0x24))(auStack_10,piVar6);
      fVar1 = pfVar8[1];
      fVar2 = pfVar8[2];
      piVar6[0xc] = (int)(*pfVar8 + unaff_EDI);
      piVar6[0xd] = (int)(fVar1 + unaff_EBX);
      piVar6[0xf] = -0x40800000;
      piVar6[0xe] = (int)(fVar2 + fVar3);
      if (piVar6[0x5c] == 2) {
        pvVar7 = (void *)FUN_005aaf90();
        FUN_005a9380(pvVar7,piVar6);
      }
    }
  }
  return;
}

