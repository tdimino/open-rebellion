
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_0059f800(void *this,float param_1)

{
  float fVar1;
  float fVar2;
  undefined4 *puVar3;
  int *piVar4;
  void *pvVar5;
  undefined4 *puVar6;
  int local_2c;
  int local_28;
  uint local_24;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651c8e;
  pvStack_c = ExceptionList;
  fVar2 = param_1 * _DAT_0066bb4c;
  local_24 = 1;
  ExceptionList = &pvStack_c;
  do {
    local_2c = -1;
    fVar1 = (float)local_24 * fVar2;
    do {
      param_1 = -NAN;
      do {
        local_28 = -1;
        do {
          if (local_24 == 1) {
            puVar3 = (undefined4 *)FUN_00618b70(0x58);
            local_4 = 6;
            if (puVar3 == (undefined4 *)0x0) {
              puVar3 = (undefined4 *)0x0;
            }
            else {
              FUN_005a6210(puVar3);
              puVar3[7] = &PTR_LAB_0066be28;
              puVar3[8] = 0;
              puVar3[9] = 0;
              puVar3[10] = &PTR_LAB_0066be24;
              puVar3[0xb] = 0;
              puVar3[0xc] = &PTR_LAB_0066be18;
              puVar3[0xd] = 0;
              puVar3[0xe] = &PTR_LAB_0066bde8;
              puVar3[0xf] = 0;
              puVar3[0x10] = 0;
              local_4 = CONCAT31(local_4._1_3_,0xb);
              FUN_0059faf0(puVar3 + 0x11);
              *puVar3 = &PTR_FUN_0066be08;
              puVar3[7] = &PTR_LAB_0066be00;
              puVar3[10] = &PTR_LAB_0066bdfc;
              puVar3[0xc] = &PTR_LAB_0066bdf0;
              puVar3[0xe] = &PTR_LAB_0066bdec;
              FUN_0059fb10(puVar3 + 0x11,(float)local_2c * fVar1,(float)(int)param_1 * fVar1,
                           (float)local_28 * fVar1);
              puVar3[0x15] = (int)param_1 % 3 + 1;
            }
            local_4 = 0xffffffff;
            piVar4 = (int *)FUN_005aaf90();
            (**(code **)(*piVar4 + 0x5c))(puVar3);
            pvVar5 = (void *)((int)this + 0x114);
LAB_0059fa59:
            FUN_005c5ca0(pvVar5,(int)puVar3);
            puVar6 = puVar3;
            pvVar5 = (void *)FUN_005aaf90();
            FUN_005a94a0(pvVar5,(int)puVar6);
            puVar3[8] = 0;
            puVar3[0xf] = 0;
            puVar3[0xb] = 0;
            puVar3[0xd] = 1;
          }
          else if (((local_2c != 0) || (param_1 != 0.0)) || (local_28 != 0)) {
            puVar3 = (undefined4 *)FUN_00618b70(0x58);
            local_4 = 0;
            if (puVar3 == (undefined4 *)0x0) {
              puVar3 = (undefined4 *)0x0;
            }
            else {
              FUN_005a6210(puVar3);
              puVar3[7] = &PTR_LAB_0066be28;
              puVar3[8] = 0;
              puVar3[9] = 0;
              puVar3[10] = &PTR_LAB_0066be24;
              puVar3[0xb] = 0;
              puVar3[0xc] = &PTR_LAB_0066be18;
              puVar3[0xd] = 0;
              local_4._0_1_ = 4;
              FUN_005a07f0(puVar3 + 0xe,0);
              local_4 = CONCAT31(local_4._1_3_,5);
              FUN_0059faf0(puVar3 + 0x11);
              *puVar3 = &PTR_FUN_0066be08;
              puVar3[7] = &PTR_LAB_0066be00;
              puVar3[10] = &PTR_LAB_0066bdfc;
              puVar3[0xc] = &PTR_LAB_0066bdf0;
              puVar3[0xe] = &PTR_LAB_0066bdec;
              FUN_0059fb10(puVar3 + 0x11,(float)local_2c * fVar1,(float)(int)param_1 * fVar1,
                           (float)local_28 * fVar1);
              puVar3[0x15] = (int)param_1 % 3 + 1;
            }
            local_4 = 0xffffffff;
            piVar4 = (int *)FUN_005aaf90();
            (**(code **)(*piVar4 + 0x5c))(puVar3);
            pvVar5 = (void *)((int)this + (local_24 + 0x16) * 0xc);
            goto LAB_0059fa59;
          }
          local_28 = local_28 + 1;
        } while (local_28 < 2);
        param_1 = (float)((int)param_1 + 1);
      } while ((int)param_1 < 2);
      local_2c = local_2c + 1;
    } while (local_2c < 2);
    local_24 = local_24 + 1;
    if (4 < local_24) {
      *(undefined4 *)((int)this + 0x144) = 0;
      *(undefined4 *)((int)this + 0x148) = 0;
      *(undefined4 *)((int)this + 0x14c) = 0;
      *(undefined4 *)((int)this + 0x150) = 0;
      ExceptionList = pvStack_c;
      return;
    }
  } while( true );
}

