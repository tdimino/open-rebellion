
undefined4 __thiscall FUN_005b7e50(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  bool bVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  undefined4 *puVar7;
  int *piVar8;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653486;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
LAB_005b7e72:
  do {
    if ((float)param_1[7] <= (float)param_1[6]) {
      if (param_1 != (undefined4 *)0x0) {
        (**(code **)*param_1)(1);
      }
      ExceptionList = pvStack_c;
      return 0;
    }
                    /* WARNING: Load size is inaccurate */
    iVar4 = (**(code **)(*this + 0xc))();
    if (iVar4 == 0) goto LAB_005b81f4;
                    /* WARNING: Load size is inaccurate */
    piVar8 = (int *)0x0;
    piVar5 = (int *)(**(code **)(*this + 0xc))();
    if (piVar5 != (int *)0x0) {
      do {
        piVar6 = piVar5;
        if ((float)piVar6[6] <= (float)param_1[6]) {
          if ((float)piVar6[6] < (float)param_1[6]) {
LAB_005b7ef9:
            bVar3 = false;
          }
          else if ((float)piVar6[7] <= (float)param_1[7]) {
            if (((float)param_1[7] != (float)piVar6[7]) || ((float)piVar6[8] <= (float)param_1[8]))
            goto LAB_005b7ef9;
            bVar3 = true;
          }
          else {
            bVar3 = true;
          }
        }
        else {
          bVar3 = true;
        }
        piVar5 = piVar6;
      } while ((!bVar3) &&
              (piVar5 = (int *)(**(code **)(*piVar6 + 8))(), piVar8 = piVar6, piVar5 != (int *)0x0))
      ;
      if (piVar5 != (int *)0x0) {
        piVar8 = (int *)(**(code **)(*piVar5 + 0xc))();
      }
    }
                    /* WARNING: Load size is inaccurate */
    piVar6 = (int *)(**(code **)(*this + 0xc))();
    if (piVar8 == piVar6) {
      if ((float)piVar8[6] <= (float)param_1[6]) {
        if ((float)piVar8[6] < (float)param_1[6]) {
LAB_005b7f7d:
          bVar3 = false;
        }
        else if ((float)piVar8[7] <= (float)param_1[7]) {
          if (((float)param_1[7] != (float)piVar8[7]) || ((float)piVar8[8] <= (float)param_1[8]))
          goto LAB_005b7f7d;
          bVar3 = true;
        }
        else {
          bVar3 = true;
        }
      }
      else {
        bVar3 = true;
      }
      if (bVar3) {
        piVar8 = (int *)0x0;
      }
    }
    if (piVar8 == (int *)0x0) {
LAB_005b80b9:
      if ((piVar5 == (int *)0x0) ||
         ((((float)piVar5[6] < (float)param_1[6] || ((float)param_1[7] <= (float)piVar5[6])) &&
          (((float)param_1[7] < (float)piVar5[7] || ((float)piVar5[7] <= (float)param_1[6]))))))
      goto LAB_005b81f4;
      if ((float)piVar5[6] <= (float)param_1[6]) {
        if (((float)param_1[6] == (float)piVar5[6]) && ((float)param_1[7] == (float)piVar5[7])) {
          piVar5[8] = (int)((float)param_1[8] + (float)piVar5[8]);
          if (param_1 == (undefined4 *)0x0) {
            ExceptionList = pvStack_c;
            return 1;
          }
          (**(code **)*param_1)(1);
          ExceptionList = pvStack_c;
          return 1;
        }
        piVar5[6] = param_1[7];
LAB_005b81eb:
        param_1[8] = (float)piVar5[8] + (float)param_1[8];
LAB_005b81f4:
        FUN_005f4f10(this,(int)param_1);
        ExceptionList = pvStack_c;
        return 1;
      }
      uVar2 = param_1[7];
      param_1[7] = piVar5[6];
      FUN_005f4f10(this,(int)param_1);
      puVar7 = (undefined4 *)FUN_00618b70(0x24);
      uStack_4 = 1;
      if (puVar7 == (undefined4 *)0x0) {
LAB_005b8176:
        uStack_4 = 0xffffffff;
        param_1 = (undefined4 *)0x0;
      }
      else {
        uVar1 = param_1[8];
        FUN_005f5c10(puVar7);
        puVar7[6] = piVar5[6];
        puVar7[7] = uVar2;
        puVar7[8] = uVar1;
        *puVar7 = &PTR_FUN_0066c678;
        uStack_4 = 0xffffffff;
        param_1 = puVar7;
      }
      goto LAB_005b7e72;
    }
    if (((float)param_1[6] == (float)piVar8[6]) && ((float)param_1[7] == (float)piVar8[7])) {
      piVar8[8] = (int)((float)param_1[8] + (float)piVar8[8]);
      if (param_1 == (undefined4 *)0x0) {
        ExceptionList = pvStack_c;
        return 1;
      }
      (**(code **)*param_1)(1);
      ExceptionList = pvStack_c;
      return 1;
    }
    if (piVar8 == (int *)0x0) goto LAB_005b80b9;
    if ((float)piVar8[6] == (float)param_1[6]) {
      piVar8[8] = (int)((float)param_1[8] + (float)piVar8[8]);
      param_1[6] = piVar8[7];
    }
    else {
      if (piVar8 == (int *)0x0) goto LAB_005b80b9;
      if ((float)piVar8[7] == (float)param_1[7]) {
        piVar8[7] = param_1[6];
        goto LAB_005b81eb;
      }
      if ((piVar8 == (int *)0x0) ||
         (((((float)piVar8[6] < (float)param_1[6] || ((float)param_1[7] <= (float)piVar8[6])) &&
           (((float)param_1[7] < (float)piVar8[7] || ((float)piVar8[7] <= (float)param_1[6])))) ||
          ((float)param_1[7] <= (float)piVar8[7])))) goto LAB_005b80b9;
      uVar2 = param_1[7];
      param_1[7] = piVar8[7];
      piVar8[7] = param_1[6];
      param_1[8] = (float)piVar8[8] + (float)param_1[8];
      FUN_005f4f10(this,(int)param_1);
      puVar7 = (undefined4 *)FUN_00618b70(0x24);
      uStack_4 = 0;
      if (puVar7 == (undefined4 *)0x0) goto LAB_005b8176;
      uVar1 = param_1[8];
      FUN_005f5c10(puVar7);
      puVar7[6] = param_1[7];
      puVar7[7] = uVar2;
      puVar7[8] = uVar1;
      *puVar7 = &PTR_FUN_0066c678;
      uStack_4 = 0xffffffff;
      param_1 = puVar7;
    }
  } while( true );
}

