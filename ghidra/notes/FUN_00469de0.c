
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_00469de0(void *this,int param_1)

{
  ushort uVar1;
  ushort uVar2;
  int *piVar3;
  void *pvVar4;
  int iVar5;
  int *piVar6;
  undefined2 extraout_var;
  undefined2 extraout_var_00;
  undefined2 extraout_var_01;
  undefined2 extraout_var_02;
  undefined2 uVar7;
  uint uVar8;
  undefined2 uStack_20;
  undefined2 uStack_1e;
  undefined2 uStack_1c;
  undefined2 uStack_1a;
  undefined4 auStack_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00631c58;
  local_c = ExceptionList;
  if (param_1 != 0) {
    piVar3 = *(int **)(param_1 + 0x68);
    ExceptionList = &local_c;
    *(undefined4 *)(param_1 + 0x38) = 10;
    iVar5 = (**(code **)(*piVar3 + 0xc))();
    if (iVar5 == 0xf) {
      FUN_006011f0((int)this);
      DAT_006b2904 = piVar3[6];
      _DAT_006b2908 = *(undefined4 *)((int)this + 0x118);
      DAT_006b290c = *(undefined4 *)((int)this + 0x11c);
      DAT_006b2910 = 0x1c;
      DAT_006b2914 = 0;
      FUN_0042a410(*(void **)((int)this + 0x114),(LPARAM)piVar3);
      (**(code **)(*(int *)this + 0x30))();
    }
    else {
      FUN_005f2f90(auStack_18,(int)(piVar3 + 0x14));
      pvVar4 = *(void **)((int)this + 0x1dc);
      uStack_4 = 0;
      FUN_005f3090((void *)((int)pvVar4 + 0xa0),(int)auStack_18);
      *(undefined4 *)((int)pvVar4 + 0x13c) = 0;
      FUN_0041fc30(pvVar4);
      (**(code **)(**(int **)((int)this + 0x1dc) + 4))(5);
      FUN_0046a200((int)this);
      pvVar4 = (void *)((int)this + 0x6c);
      piVar6 = (int *)FUN_00604500(pvVar4,0x97);
      (**(code **)(*piVar6 + 8))();
      piVar6 = (int *)FUN_00604500(pvVar4,0x98);
      (**(code **)(*piVar6 + 8))();
      piVar6 = (int *)FUN_00604500(pvVar4,0x99);
      (**(code **)(*piVar6 + 8))();
      iVar5 = (**(code **)(*piVar3 + 0x10))();
      if (iVar5 == 5) {
        iVar5 = 0x140;
      }
      else {
        iVar5 = 0x18b;
      }
      SetWindowPos(*(HWND *)(*(int *)((int)this + 0x1dc) + 0x18),(HWND)0x0,0,0,iVar5,
                   *(int *)(*(int *)((int)this + 0x1dc) + 0x34),6);
      iVar5 = (**(code **)(*piVar3 + 0x10))();
      if (iVar5 == 3) {
        piVar6 = (int *)FUN_00604500(pvVar4,0x67);
        if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
          FUN_00603150(piVar6,0,0x2916);
          uVar8 = 0x2917;
        }
        else {
          FUN_00603150(piVar6,0,0x2918);
          uVar8 = 0x2919;
        }
        FUN_00603150(piVar6,1,uVar8);
        uStack_1c = 0x8030;
        uStack_1a = DAT_0065d424;
        FUN_00600c40(this,piVar6,&uStack_1c);
        (**(code **)(*piVar6 + 4))(5);
        iVar5 = (**(code **)(*piVar6 + 0xc))();
        uVar7 = extraout_var_02;
      }
      else if (iVar5 == 4) {
        piVar6 = (int *)FUN_00604500(pvVar4,0x67);
        if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
          FUN_00603150(piVar6,0,0x2d20);
          uVar8 = 0x2d21;
        }
        else {
          FUN_00603150(piVar6,0,0x2d47);
          uVar8 = 0x2d48;
        }
        FUN_00603150(piVar6,1,uVar8);
        uStack_20 = 0x8031;
        uStack_1e = DAT_0065d424;
        FUN_00600c40(this,piVar6,&uStack_20);
        (**(code **)(*piVar6 + 4))(5);
        iVar5 = (**(code **)(*piVar6 + 0xc))();
        uVar7 = extraout_var_01;
      }
      else {
        iVar5 = iVar5 + -5;
        uVar7 = extraout_var;
        if (iVar5 == 0) {
          iVar5 = FUN_004ece60((uint *)(piVar3 + 0x17));
          if (iVar5 == 0) {
            piVar6 = (int *)FUN_00604500(pvVar4,0x67);
            (**(code **)(*piVar6 + 0x10))();
          }
          else {
            if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
              FUN_00603150(piVar6,0,0x2916);
              uVar8 = 0x2917;
            }
            else {
              FUN_00603150(piVar6,0,0x2918);
              uVar8 = 0x2919;
            }
            FUN_00603150(piVar6,1,uVar8);
            param_1 = CONCAT22(DAT_0065d424,0x8030);
            FUN_00600c40(this,piVar6,&param_1);
            (**(code **)(*piVar6 + 4))(5);
            (**(code **)(*piVar6 + 0xc))();
          }
          piVar6 = (int *)FUN_00604500(pvVar4,0x98);
          FUN_00603150(piVar6,0,0x2aae);
          FUN_00603150(piVar6,1,0x2aaf);
          (**(code **)(*piVar6 + 4))(5);
          piVar6 = (int *)FUN_00604500(pvVar4,0x99);
          FUN_00603150(piVar6,0,0x2ab1);
          FUN_00603150(piVar6,1,0x2ab2);
          iVar5 = (**(code **)(*piVar6 + 4))(5);
          uVar7 = extraout_var_00;
        }
      }
      FUN_0046a320(this,CONCAT22(uVar7,(short)piVar3[0xb]),
                   CONCAT22((short)((uint)iVar5 >> 0x10),*(undefined2 *)((int)piVar3 + 0x2e)));
      uVar1 = *(ushort *)(piVar3 + 0xc);
      uVar2 = *(ushort *)((int)piVar3 + 0x32);
      FUN_0046a6e0((int)this);
      *(uint *)((int)this + 300) = (uint)uVar1;
      if (uVar1 != 0) {
        *(undefined4 *)((int)this + 0x130) = 7;
        *(uint *)((int)this + 0x138) = *(uint *)((int)this + 0x138) & 0xfffffff9 | 1;
        FUN_00610c30((int)this + 0x128);
      }
      *(uint *)((int)this + 0x144) = (uint)uVar2;
      if (uVar2 != 0) {
        *(undefined4 *)((int)this + 0x148) = 9;
        *(uint *)((int)this + 0x150) = *(uint *)((int)this + 0x150) & 0xfffffff9 | 1;
        FUN_00610c30((int)this + 0x140);
      }
      uStack_4 = 0xffffffff;
      FUN_005f2ff0(auStack_18);
    }
  }
  ExceptionList = local_c;
  return;
}

