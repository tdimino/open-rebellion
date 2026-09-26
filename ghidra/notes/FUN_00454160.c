
void __thiscall FUN_00454160(void *this,uint *param_1,void *param_2)

{
  void *this_00;
  uint *this_01;
  bool bVar1;
  uint uVar2;
  ushort uVar3;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  void *pvVar7;
  undefined4 *puVar8;
  int iVar9;
  uint *puVar10;
  uint *local_34;
  void *local_30;
  void *local_2c;
  uint local_28;
  RECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ff70;
  local_c = ExceptionList;
  bVar1 = false;
  this_00 = (void *)((int)this + 0x114);
  local_34 = (uint *)0x0;
  local_30 = (void *)0x0;
  ExceptionList = &local_c;
  FUN_004f5940(this_00,param_1);
  FUN_006037f0(7);
  local_2c = (void *)0x0;
  local_28 = 0;
  piVar4 = FUN_004a25c0(this);
  if (piVar4 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  local_1c.left = 0x2c;
  local_1c.top = 0x2d;
  if ((0x2b < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x2d)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    if (*(int *)((int)this + 0x160) == 0x6c) {
      local_34 = FUN_00458fe0(param_2);
      if ((*(byte *)((int)param_2 + 0x50) & 4) == 0) {
        iVar6 = FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
        if (iVar6 == 0) {
          pvVar7 = (void *)FUN_00618b70(0x74);
          local_4 = 0;
          if (pvVar7 == (void *)0x0) {
            local_30 = (void *)0x0;
          }
          else {
            local_30 = FUN_004421d0(pvVar7,param_1);
          }
          local_4 = 0xffffffff;
          pvVar7 = (void *)FUN_00618b70(0x28);
          local_4 = 1;
          if (pvVar7 == (void *)0x0) {
            pvVar7 = (void *)0x0;
          }
          else {
            pvVar7 = FUN_005fbda0(pvVar7,0x45,0x28,local_34);
          }
          local_4 = 0xffffffff;
          pvVar5 = (void *)FUN_00618b70(0x28);
          local_4 = 2;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            pvVar5 = FUN_005fbda0(pvVar5,0x45,0x28,local_34);
          }
          local_4 = 0xffffffff;
          FUN_0060be60(local_30,(int)pvVar7,(int)pvVar5);
          FUN_005f59f0(*(void **)((int)this + 0x178),(int)local_30);
        }
      }
      else {
        local_30 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
        *(uint *)((int)local_30 + 0x3c) = *(uint *)((int)local_30 + 0x3c) & 0xfffffffb;
      }
    }
    local_2c = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6c);
    local_28 = 0x2851;
  }
  else {
    local_1c.left = 0x2d;
    local_1c.top = 0x2e;
    if ((*param_1 >> 0x18 < 0x2d) || (0x2d < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      if (*(int *)((int)this + 0x160) == 0x6b) {
        local_34 = FUN_00458fe0(param_2);
        if ((*(byte *)((int)param_2 + 0x50) & 4) == 0) {
          iVar6 = FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
          if (iVar6 == 0) {
            pvVar7 = (void *)FUN_00618b70(0x74);
            local_4 = 3;
            if (pvVar7 == (void *)0x0) {
              local_30 = (void *)0x0;
            }
            else {
              local_30 = FUN_004421d0(pvVar7,param_1);
            }
            local_4 = 0xffffffff;
            pvVar7 = (void *)FUN_00618b70(0x28);
            local_4 = 4;
            if (pvVar7 == (void *)0x0) {
              pvVar7 = (void *)0x0;
            }
            else {
              puVar10 = local_34;
              iVar6 = FUN_005fc0f0((int *)local_34);
              iVar9 = FUN_005fc0e0((int *)local_34);
              pvVar7 = FUN_005fbda0(pvVar7,iVar9,iVar6,puVar10);
            }
            local_4 = 0xffffffff;
            pvVar5 = (void *)FUN_00618b70(0x28);
            local_4 = 5;
            if (pvVar5 == (void *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar10 = local_34;
              iVar6 = FUN_005fc0f0((int *)local_34);
              iVar9 = FUN_005fc0e0((int *)local_34);
              pvVar5 = FUN_005fbda0(pvVar5,iVar9,iVar6,puVar10);
            }
            local_4 = 0xffffffff;
            FUN_0060be60(local_30,(int)pvVar7,(int)pvVar5);
            FUN_005f59f0(*(void **)((int)this + 0x178),(int)local_30);
          }
        }
        else {
          local_30 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
        }
      }
      local_2c = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6b);
      local_28 = 0x2854;
    }
    else {
      local_1c.left = 0x28;
      local_1c.top = 0x29;
      if ((*param_1 >> 0x18 < 0x28) || (0x28 < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (*(int *)((int)this + 0x160) == 0x68) {
          local_34 = FUN_00458fe0(param_2);
          if ((*(byte *)((int)param_2 + 0x50) & 4) == 0) {
            iVar6 = FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
            if (iVar6 == 0) {
              pvVar7 = (void *)FUN_00618b70(0x74);
              local_4 = 6;
              if (pvVar7 == (void *)0x0) {
                local_30 = (void *)0x0;
              }
              else {
                local_30 = FUN_004421d0(pvVar7,param_1);
              }
              local_4 = 0xffffffff;
              pvVar7 = (void *)FUN_00618b70(0x28);
              local_4 = 7;
              if (pvVar7 == (void *)0x0) {
                pvVar7 = (void *)0x0;
              }
              else {
                pvVar7 = FUN_005fbda0(pvVar7,0x45,0x28,local_34);
              }
              local_4 = 0xffffffff;
              pvVar5 = (void *)FUN_00618b70(0x28);
              local_4 = 8;
              if (pvVar5 == (void *)0x0) {
                pvVar5 = (void *)0x0;
              }
              else {
                pvVar5 = FUN_005fbda0(pvVar5,0x45,0x28,local_34);
              }
              local_4 = 0xffffffff;
              FUN_0060be60(local_30,(int)pvVar7,(int)pvVar5);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)local_30);
            }
          }
          else {
            local_30 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
          }
        }
        else if (*(int *)((int)this + 0x160) == 0x67) {
          local_1c.left = 6;
          local_1c.top = 0x34;
          local_1c.right = 0x47;
          local_1c.bottom = 0x87;
          InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
        }
        local_2c = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x68);
        local_28 = 0x2857;
      }
      else {
        local_1c.left = 0x29;
        local_1c.top = 0x2a;
        if ((*param_1 >> 0x18 < 0x29) || (0x29 < *param_1 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          if (*(int *)((int)this + 0x160) == 0x69) {
            local_34 = FUN_00458fe0(param_2);
            if ((*(byte *)((int)param_2 + 0x50) & 4) == 0) {
              iVar6 = FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
              if (iVar6 == 0) {
                pvVar7 = (void *)FUN_00618b70(0x74);
                local_4 = 9;
                if (pvVar7 == (void *)0x0) {
                  local_30 = (void *)0x0;
                }
                else {
                  local_30 = FUN_004421d0(pvVar7,param_1);
                }
                local_4 = 0xffffffff;
                pvVar7 = (void *)FUN_00618b70(0x28);
                local_4 = 10;
                if (pvVar7 == (void *)0x0) {
                  pvVar7 = (void *)0x0;
                }
                else {
                  pvVar7 = FUN_005fbda0(pvVar7,0x45,0x28,local_34);
                }
                local_4 = 0xffffffff;
                pvVar5 = (void *)FUN_00618b70(0x28);
                local_4 = 0xb;
                if (pvVar5 == (void *)0x0) {
                  pvVar5 = (void *)0x0;
                }
                else {
                  pvVar5 = FUN_005fbda0(pvVar5,0x45,0x28,local_34);
                }
                local_4 = 0xffffffff;
                FUN_0060be60(local_30,(int)pvVar7,(int)pvVar5);
                FUN_005f59f0(*(void **)((int)this + 0x178),(int)local_30);
              }
            }
            else {
              local_30 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
            }
          }
          else if (*(int *)((int)this + 0x160) == 0x67) {
            local_1c.left = 6;
            local_1c.top = 0x34;
            local_1c.right = 0x47;
            local_1c.bottom = 0xd8;
            InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
          }
          local_2c = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x69);
          local_28 = 0x285a;
        }
        else {
          local_1c.left = 0x2a;
          local_1c.top = 0x2b;
          if ((*param_1 >> 0x18 < 0x2a) || (0x2a < *param_1 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            if (*(int *)((int)this + 0x160) == 0x6a) {
              local_34 = FUN_00458fe0(param_2);
              if ((*(byte *)((int)param_2 + 0x50) & 4) == 0) {
                iVar6 = FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
                if (iVar6 == 0) {
                  pvVar7 = (void *)FUN_00618b70(0x74);
                  local_4 = 0xc;
                  if (pvVar7 == (void *)0x0) {
                    local_30 = (void *)0x0;
                  }
                  else {
                    local_30 = FUN_004421d0(pvVar7,param_1);
                  }
                  local_4 = 0xffffffff;
                  pvVar7 = (void *)FUN_00618b70(0x28);
                  local_4 = 0xd;
                  if (pvVar7 == (void *)0x0) {
                    pvVar7 = (void *)0x0;
                  }
                  else {
                    pvVar7 = FUN_005fbda0(pvVar7,0x45,0x28,local_34);
                  }
                  local_4 = 0xffffffff;
                  pvVar5 = (void *)FUN_00618b70(0x28);
                  local_4 = 0xe;
                  if (pvVar5 == (void *)0x0) {
                    pvVar5 = (void *)0x0;
                  }
                  else {
                    pvVar5 = FUN_005fbda0(pvVar5,0x45,0x28,local_34);
                  }
                  local_4 = 0xffffffff;
                  FUN_0060be60(local_30,(int)pvVar7,(int)pvVar5);
                  FUN_005f59f0(*(void **)((int)this + 0x178),(int)local_30);
                }
              }
              else {
                local_30 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*param_1 & 0xffffff);
              }
            }
            else if (*(int *)((int)this + 0x160) == 0x67) {
              local_1c.left = 6;
              local_1c.top = 0x34;
              local_1c.right = 0x47;
              local_1c.bottom = 0x128;
              InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
            }
            local_2c = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),0x6a);
            local_28 = 0x285d;
          }
        }
      }
    }
  }
  if (local_30 == (void *)0x0) {
    if (local_34 == (uint *)0x0) goto LAB_00454a89;
    FUN_005fbfa0(local_34);
  }
  else {
    if (local_34 == (uint *)0x0) goto LAB_00454a89;
    puVar10 = *(uint **)((int)local_30 + 0x20);
    this_01 = *(uint **)((int)local_30 + 0x24);
    iVar6 = FUN_005fc0e0((int *)local_34);
    iVar9 = (0x45 - iVar6) / 2;
    iVar6 = FUN_005fc0f0((int *)local_34);
    iVar6 = (0x28 - iVar6) / 2;
    uVar2 = local_34[2];
    if (puVar10 != (uint *)0x0) {
      FUN_005fcfe0(puVar10,0,0,0x45,0x28,(char)uVar2);
      FUN_005fcc30(local_34,puVar10,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
    }
    if (this_01 != (uint *)0x0) {
      FUN_005fcfe0(this_01,0,0,0x45,0x28,(char)uVar2);
      FUN_005fcc30(local_34,this_01,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
    }
    FUN_005fd0f0(*(void **)((int)this + 0x15c),this_01,iVar9,iVar6);
    FUN_0060a280(*(void **)((int)this + 0x174));
    FUN_005fbfa0(local_34);
  }
  FUN_00618b60((undefined *)local_34);
LAB_00454a89:
  pvVar7 = FUN_004f5940(this_00,param_1);
  if (pvVar7 == (void *)0x0) {
    puVar8 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = 0xf;
    if (puVar8 == (undefined4 *)0x0) {
      puVar8 = (undefined4 *)0x0;
    }
    else {
      puVar8 = FUN_004acd10(puVar8);
    }
    local_4 = 0xffffffff;
    FUN_004f26d0(puVar8 + 6,param_1);
    FUN_004acba0(this_00,puVar8,0);
  }
  if (((local_2c != (void *)0x0) && (local_28 != 0)) &&
     (uVar3 = FUN_006034a0(local_2c,0), uVar3 != local_28)) {
    FUN_00603150(local_2c,0,local_28);
    InvalidateRect(*(HWND *)((int)local_2c + 0x18),(RECT *)0x0,0);
  }
  ExceptionList = local_c;
  return;
}

